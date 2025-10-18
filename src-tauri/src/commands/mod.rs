use crate::state::AppState;
use crate::utils::app_data_dir::AppDataDir;
use crate::utils::check_file_exists::CheckFileExists;
use crate::utils::parse_otpauth;
use log::{error, info};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ConnectionTrait, Database, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QuerySelect,
};
use sea_orm_cli::cli;
use std::fs;
use std::sync::{Arc, Mutex};
use tauri::{Manager, State};
use xauthenticator_entity::account::{ActiveModel, Entity as AccountEntity, Model};
use xauthenticator_entity::{PageParam, Response};
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
    if !password.is_empty() {
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
pub fn add_account(app: tauri::AppHandle, auth_url: String) {
    if auth_url.trim().is_empty() {
        error!("add_account: auth_url is empty");
        return;
    }

    // Clone the DB connection without holding the mutex across potential await
    let db = {
        let state = app.state::<Arc<Mutex<AppState>>>();
        let state = state.lock().unwrap();
        match state.db.as_ref().cloned() {
            Some(db) => db,
            None => {
                error!("add_account: database not initialized");
                return;
            }
        }
    };

    // Parse otpauth URL
    let parsed = match parse_otpauth::parse_otpauth(&auth_url) {
        Some(p) => p,
        None => {
            error!("add_account: failed to parse otpauth url: {}", auth_url);
            return;
        }
    };

    // Build ActiveModel (store secret bytes as-is for now; nonce empty)
    let account = ActiveModel {
        id: Set(uuid::Uuid::new_v7(uuid::Timestamp::now(uuid::NoContext)).to_string()),
        issuer: Set(parsed.issuer),
        label: Set(parsed.label),
        type_: Set(parsed.r#type),
        algorithm: Set(parsed.algorithm),
        digits: Set(parsed.digits),
        period: Set(parsed.period),
        counter: Set(parsed.counter),
        secret_cipher: Set(parsed.secret),
        secret_nonce: Set(Vec::new()),
        icon: Set(None),
        note: Set(None),
        created_at: Set(None),
        updated_at: Set(None),
    };

    // Execute insertion
    let res = tauri::async_runtime::block_on(async {
        xauthenticator_repository::account::add_account(&db, account).await
    });

    match res {
        Ok(model) => info!("add_account: inserted account id={}", model.id),
        Err(e) => error!("add_account: failed to insert account: {:?}", e),
    }
}

#[tauri::command]
pub fn remove_account(app: tauri::AppHandle, account_id: uuid::Uuid) {}

#[tauri::command]
pub fn export_backup(app: tauri::AppHandle, password: String) -> Vec<u8> {
    Vec::new()
}

#[tauri::command]
pub fn import_backup(app: tauri::AppHandle, backup: Vec<u8>, password: String) {}

#[tauri::command]
pub fn get_code(app: tauri::AppHandle, account_id: uuid::Uuid) {}

#[tauri::command]
pub fn health_check(app: tauri::AppHandle) {}
