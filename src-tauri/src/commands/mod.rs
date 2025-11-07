use crate::state::AppState;
use crate::utils::app_data_dir::AppDataDir;
use crate::utils::{crypto, kdf, parse_otpauth};
use log::{error, info};
use sea_orm::ActiveValue::Set;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_cli::cli;
use serde::Serialize;
use std::fs;
use std::io::Write;
use std::sync::{Arc, Mutex};
use tauri::Manager;
use xauthenticator_entity::account::ActiveModel;
use xauthenticator_entity::PageParam;
use xauthenticator_entity::Response;
use xauthenticator_error::CommonError;

#[tauri::command]
pub fn init_app(app: tauri::AppHandle, password: String) -> Result<(), CommonError> {
    let app_data_dir = AppDataDir::new(
        app.path()
            .app_local_data_dir()
            .expect("could not resolve app local data path"),
    );
    let salt_path = app_data_dir.salt();

    app.plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())
        .expect("failed to initialize stronghold plugin");

    let has_min_len = password.len() >= 12;
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password
        .chars()
        .any(|c| !c.is_ascii_alphanumeric() && !c.is_whitespace());
    if !(has_min_len && has_upper && has_lower && has_digit && has_special) {
        return Err(CommonError::InvalidPassword);
    }

    let master_key_path = app_data_dir.master_key();
    if master_key_path.exists() {
        info!("master key already initialized");
        return Ok(());
    }

    let salt: Vec<u8> = if salt_path.exists() {
        fs::read(&salt_path).expect("failed to read salt")
    } else {
        let mut salt_buf = [0u8; 16];
        getrandom::fill(&mut salt_buf).expect("failed to generate salt");
        let mut f = fs::File::create(&salt_path).expect("failed to create salt file");
        f.write_all(&salt_buf).expect("failed to write salt file");
        salt_buf.to_vec()
    };

    let derived = kdf::derive_key(&password, Some(&salt));

    let mut master_key = [0u8; 32];
    getrandom::fill(&mut master_key).expect("failed to generate master key");
    let (nonce, ciphertext) = crypto::encrypt_xchacha20poly1305(&master_key, &derived.0)
        .map_err(|e| CommonError::UnexpectedError(e.into()))?;

    let mut f = fs::File::create(&master_key_path).expect("failed to create master key file");
    f.write_all(&nonce)
        .expect("failed to write master key nonce");
    f.write_all(&ciphertext)
        .expect("failed to write master key ciphertext");
    info!("master key initialized and stored securely");

    Ok(())
}
#[tauri::command]
pub fn launch_app(app: tauri::AppHandle) -> Result<(), CommonError> {
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
        return Err(CommonError::MasterKeyNotInitialized);
    } else {
        info!("master key storage found; deferring load until unlock");
        app_state.is_locked = true;
        app_state.master_key = None;
    }

    let cfg = xauthenticator_config::Config::init(app_data_dir.config()).load();

    app_state.config = cfg;

    app_state.is_initialized = true;

    app_state.runtime_timestamp = chrono::Local::now().timestamp() as u64;

    info!("app initialized");

    Ok(())
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
    let app_data_dir = AppDataDir::new(
        app.path()
            .app_local_data_dir()
            .expect("could not resolve app local data path"),
    );

    // 读取盐并派生密钥
    let salt = fs::read(app_data_dir.salt()).map_err(|e| CommonError::UnexpectedError(e.into()))?;
    let derived = kdf::derive_key(&password, Some(&salt));

    // 读取并解密主密钥
    let enc =
        fs::read(app_data_dir.master_key()).map_err(|e| CommonError::UnexpectedError(e.into()))?;
    if enc.len() < 24 + 16 {
        error!("invalid master key blob size: {}", enc.len());
        return Err(CommonError::InvalidMasterKey);
    }
    let (nonce, ciphertext) = enc.split_at(24);
    let plaintext = crypto::decrypt_xchacha20poly1305(&ciphertext, &nonce, &derived.0)
        .map_err(|e| CommonError::UnexpectedError(e.into()))?;
    if plaintext.len() != 32 {
        error!("invalid decrypted master key size: {}", plaintext.len());
        return Err(CommonError::InvalidMasterKey);
    }
    let mut key_buf = [0u8; 32];
    key_buf.copy_from_slice(&plaintext);

    let state = app.state::<Arc<Mutex<AppState>>>();
    let mut state = state.lock().unwrap();
    state.master_key = Some(key_buf);
    state.is_locked = false;
    state.locked_timestamp = None;
    Ok(())
}

#[tauri::command]
pub fn unlock_with_biometric(app: tauri::AppHandle) {}

#[tauri::command]
pub fn lock(app: tauri::AppHandle) -> Result<(), String> {
    let state = app.state::<Arc<Mutex<AppState>>>();
    let mut state = state.lock().unwrap();
    state.is_locked = true;
    state.locked_timestamp = Some(chrono::Local::now().timestamp() as u64);
    state.master_key = None;
    Ok(())
}

#[tauri::command]
pub async fn list_accounts(
    app: tauri::AppHandle,
    current: u32,
    size: u32,
) -> Result<Response<Vec<xauthenticator_entity::response::Account>>, CommonError> {
    // Gate: must be unlocked
    {
        let state = app.state::<Arc<Mutex<AppState>>>();
        let state = state.lock().unwrap();
        if state.is_locked || state.master_key.is_none() {
            return Err(CommonError::AppIsLocked);
        }
    }
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
    let resp = xauthenticator_repository::account::list_accounts(&db, PageParam { current, size })
        .await
        .expect("failed to list accounts");

    Ok(Response {
        data: resp
            .data
            .iter()
            .map(|v| xauthenticator_entity::response::Account {
                id: v.id.to_string(),
                issuer: v.issuer.to_string(),
                label: v.label.to_string(),
                r#type: v.r#type.to_string(),
                algorithm: v.algorithm.to_string(),
                digits: v.digits,
                period: v.period,
                counter: v.counter,
                secret_cipher: v.secret_cipher.clone(),
            })
            .collect(),
        total: resp.total,
    })
}

#[tauri::command]
pub fn add_account(app: tauri::AppHandle, auth_url: String) -> Result<(), CommonError> {
    if auth_url.trim().is_empty() {
        error!("add_account: auth_url is empty");
        return Err(CommonError::RequestError("auth_url is empty".to_string()));
    }
    info!("add_account: auth_url={}", auth_url);
    // Gate: must be unlocked
    let state = app.state::<Arc<Mutex<AppState>>>();
    let state = state.lock().unwrap();
    if state.is_locked || state.master_key.is_none() {
        return Err(CommonError::AppIsLocked);
    }
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
        r#type: Set(parsed.r#type),
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

    let res = tauri::async_runtime::block_on(async {
        xauthenticator_repository::account::add_account(&db, account).await
    })
    .expect("failed to add account");

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
    // Gate: must be unlocked
    let (db, is_locked) = {
        let state = app.state::<Arc<Mutex<AppState>>>();
        let state = state.lock().unwrap();
        (
            state.db.as_ref().cloned().unwrap(),
            state.is_locked || state.master_key.is_none(),
        )
    };
    if is_locked {
        return Err(CommonError::AppIsLocked);
    }
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
pub fn get_code(app: tauri::AppHandle, account_id: uuid::Uuid) -> Result<String, CommonError> {
    // Gate: must be unlocked
    {
        let state = app.state::<Arc<Mutex<AppState>>>();
        let state = state.lock().unwrap();
        if state.is_locked || state.master_key.is_none() {
            return Err(CommonError::AppIsLocked);
        }
    }
    // Implementation of code generation would go here
    Err(CommonError::RequestError("not implemented".to_string()))
}

#[tauri::command]
pub fn health_check(app: tauri::AppHandle) -> Result<(), CommonError> {
    let state = app.state::<Arc<Mutex<AppState>>>();
    let mut state = state.lock().unwrap();
    let timeout = state.config.builder().settings.auto_lock_timeout;
    let auto_lock_enabled = state.config.builder().settings.auto_lock;
    if auto_lock_enabled && !state.is_locked {
        let now = chrono::Local::now().timestamp() as u64;
        if now.saturating_sub(state.runtime_timestamp) >= timeout {
            state.is_locked = true;
            state.locked_timestamp = Some(now);
            state.master_key = None;
            return Err(CommonError::AppIsLocked);
        }
    }
    Ok(())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthCapabilities {
    pub biometric_supported: bool,
    pub pin_supported: bool,
    pub methods: Vec<String>,
}

#[tauri::command]
pub fn auth_capabilities(_app: tauri::AppHandle) -> Result<AuthCapabilities, CommonError> {
    let os = std::env::consts::OS;
    let mut methods: Vec<String> = Vec::new();
    let mut biometric = false;
    let mut pin = false;
    match os {
        "windows" => {
            // 未来可集成 Windows Hello/FIDO2 检测
            biometric = false;
            pin = true; // Windows 通常支持系统 PIN
            methods.push("PIN".to_string());
        }
        "macos" => {
            biometric = true; // Touch ID 常见
            methods.push("Touch ID".to_string());
        }
        "linux" => {
            // 未来可检测 libpam/fprintd
            biometric = false;
            pin = false;
        }
        "android" | "ios" => {
            biometric = true;
            methods.push("Biometric".to_string());
        }
        _ => {}
    }
    Ok(AuthCapabilities {
        biometric_supported: biometric,
        pin_supported: pin,
        methods,
    })
}

#[tauri::command]
pub fn quit_app(app: tauri::AppHandle) {
    app.exit(0);
}
