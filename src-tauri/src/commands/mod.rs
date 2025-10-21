use crate::state::AppState;
use crate::utils::app_data_dir::AppDataDir;
use crate::utils::{crypto, parse_otpauth};
use log::{error, info};
use sea_orm::ActiveValue::Set;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_cli::cli;
use std::fs;
use std::sync::{Arc, Mutex};
use tauri::Manager;
use xauthenticator_entity::account::ActiveModel;
use xauthenticator_entity::account::Model;
use xauthenticator_entity::PageParam;
use xauthenticator_entity::Response;
use xauthenticator_error::CommonError;

#[tauri::command]
pub fn init_app(app: tauri::AppHandle) {
    let state = app.state::<Arc<Mutex<AppState>>>();
    let mut app_state = state.lock().unwrap();

    let current_version = format!("v{}", app.config().version.clone().unwrap());
    info!("app config version: {:?}", current_version);
    let app_data_dir = AppDataDir::new(
        app.path()
            .app_local_data_dir()
            .expect("could not resolve app local data path"),
    );

    if !app_data_dir.app().exists() {
        fs::create_dir_all(&app_data_dir.app()).expect("failed to create app data directory");
    }

    let version_path = app_data_dir.version();
    let first_time = if !version_path.exists() {
        // 情况1: version.txt 不存在，创建文件并写入当前版本
        info!("version.txt not found, creating new file");
        fs::write(&version_path, current_version.as_str())
            .expect("failed to write version to version.txt");
        info!(
            "created version.txt file: {} and wrote version {}",
            version_path.to_str().unwrap(),
            current_version
        );
        true
    } else {
        // 情况2: version.txt 存在，读取并比对版本
        let stored_version =
            fs::read_to_string(&version_path).expect("failed to read version.txt file");
        info!("stored version: {}", stored_version);
        if stored_version.trim() == current_version.as_str() {
            false
        } else {
            // 版本不一致，更新版本文件
            info!(
                "version mismatch: stored={}, current={}, updating version.txt",
                stored_version.trim(),
                current_version
            );
            fs::write(&version_path, current_version.as_str())
                .expect("failed to update version in version.txt");
            info!(
                "updated version.txt file: {} to: {}",
                version_path.to_str().unwrap(),
                current_version
            );
            true
        }
    };

    let db_path = app_data_dir.db(app.config().identifier.to_lowercase().clone());
    if !db_path.exists() {
        info!("database file not found, creating new file");
        fs::File::create(&db_path).expect("failed to create database file");
        info!("created database file: {}", db_path.to_str().unwrap());
    }

    tauri::async_runtime::block_on(async {
        let db: DatabaseConnection =
            Database::connect(format!("sqlite:{}", db_path.to_str().unwrap()))
                .await
                .expect("failed to connect to database");

        if first_time {
            info!("running migrations...");
            xauthenticator_migration::run_migrate(
                &db,
                Option::from(cli::MigrateSubcommands::Up { num: None }),
                false,
            )
            .await
            .expect("failed to run migrations");
            info!("migrations completed");
        }

        app_state.db = Some(db);
    });

    let master_key_path = app_data_dir.master_key();
    if !master_key_path.exists() {
        info!("master key file not found, creating new file");
        let mut master_key = [0u8; 32];
        getrandom::fill(&mut master_key).expect("failed to generate random key");
        fs::write(&master_key_path, &master_key).expect("failed to write master key");
        info!(
            "created master key file: {}",
            master_key_path.to_str().unwrap()
        );
        app_state.master_key = Some(master_key);
    } else {
        info!("master key file found");
        let data = fs::read(&master_key_path).expect("failed to read master key");
        if data.len() != 32 {
            error!("invalid master key size: {}", data.len());
            return;
        }
        info!("master key file loaded");
        let mut key = [0u8; 32];
        key.copy_from_slice(&data);
        app_state.master_key = Some(key);
    }

    let cfg = xauthenticator_config::Config::init(app_data_dir.config()).load();

    app_state.config = cfg;

    app_state.is_initialized = true;

    info!("app initialized");
}

#[tauri::command]
pub fn app_state(app: tauri::AppHandle) -> Result<AppState, CommonError> {
    let app_state = app.state::<Arc<Mutex<AppState>>>();
    let app_state = app_state.lock().unwrap();
    Ok(app_state.clone())
}

#[tauri::command]
pub fn unlock_with_password(app: tauri::AppHandle, password: String) -> Result<(), CommonError> {
    if password.is_empty() {
        return Err(CommonError::RequestError("password is empty".to_string()));
    }

    let state = app.state::<Arc<Mutex<AppState>>>();
    let mut state = state.lock().unwrap();
    state.is_locked = false;
    Ok(())
}

#[tauri::command]
pub fn unlock_with_biometric(app: tauri::AppHandle) {}

#[tauri::command]
pub fn lock(app: tauri::AppHandle) -> Result<(), String> {
    let state = app.state::<Arc<Mutex<AppState>>>();
    let mut state = state.lock().unwrap();
    state.is_locked = true;
    Ok(())
}

#[tauri::command]
pub async fn list_accounts(
    app: tauri::AppHandle,
    param: PageParam,
) -> Result<Response<Vec<Model>>, CommonError> {
    // Get a clone of the DatabaseConnection without holding the mutex across await
    let db = {
        let state = app.state::<Arc<Mutex<AppState>>>();
        let state = state.lock().unwrap();
        state
            .db
            .as_ref()
            .cloned()
            .ok_or_else(|| CommonError::RequestError("database not initialized".to_string()))?
    };

    Ok(xauthenticator_repository::account::list_accounts(&db, param).await?)
}

#[tauri::command]
pub async fn add_account(app: tauri::AppHandle, auth_url: String) -> Result<(), CommonError> {
    if auth_url.trim().is_empty() {
        error!("add_account: auth_url is empty");
        return Err(CommonError::RequestError("auth_url is empty".to_string()));
    }
    info!("add_account: auth_url={}", auth_url);
    let state = app.state::<Arc<Mutex<AppState>>>();
    let state = state.lock().unwrap();
    // Clone the DB connection without holding the mutex across potential await
    let db = state.db.as_ref().cloned().unwrap();
    let master_key = state.master_key.as_ref().cloned().unwrap();

    // Parse otpauth URL
    let parsed = parse_otpauth::parse_otpauth(&auth_url).expect("failed to parse otpauth URL");
    let (nonce, ciphertext) = crypto::encrypt_xchacha20poly1305(&parsed.secret, &master_key)
        .expect("failed to encrypt secret");

    // Build ActiveModel with encrypted secret + nonce
    let account = ActiveModel {
        id: Set(uuid::Uuid::new_v7(uuid::Timestamp::now(uuid::NoContext)).to_string()),
        issuer: Set(parsed.issuer),
        label: Set(parsed.label),
        type_: Set(parsed.r#type),
        algorithm: Set(parsed.algorithm),
        digits: Set(parsed.digits),
        period: Set(parsed.period),
        counter: Set(parsed.counter),
        secret_cipher: Set(ciphertext),
        secret_nonce: Set(nonce),
        icon: Set(None),
        note: Set(None),
        created_at: Set(None),
        updated_at: Set(None),
    };

    // Execute insertion
    let res = xauthenticator_repository::account::add_account(&db, account)
        .await
        .expect("failed to insert account");

    info!("add_account: res={:?}", res);

    Ok(())
}

#[tauri::command]
pub async fn remove_account(
    app: tauri::AppHandle,
    account_id: uuid::Uuid,
) -> Result<(), CommonError> {
    if account_id.is_nil() {
        error!("remove_account: account_id is nil");
        return Err(CommonError::RequestError("account_id is nil".to_string()));
    }
    info!("remove_account: account_id={}", account_id);
    let db = {
        let state = app.state::<Arc<Mutex<AppState>>>();
        let state = state.lock().unwrap();
        let db = state.db.as_ref().cloned().unwrap();
        db
    };
    xauthenticator_repository::account::remove_account(&db, account_id.to_string())
        .await
        .expect("failed to remove account");

    Ok(())
}

#[tauri::command]
pub fn export_backup(app: tauri::AppHandle, password: String) {}

#[tauri::command]
pub fn import_backup(app: tauri::AppHandle, backup: Vec<u8>, password: String) {}

#[tauri::command]
pub fn get_code(app: tauri::AppHandle, account_id: uuid::Uuid) {}

#[tauri::command]
pub fn health_check(app: tauri::AppHandle) {}

#[tauri::command]
pub fn quit_app(app: tauri::AppHandle) {
    // Quit the entire application with exit code 0
    app.exit(0);
}
