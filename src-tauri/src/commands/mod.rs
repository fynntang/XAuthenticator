use crate::state::AppState;
use crate::utils;
use crate::utils::app_data_dir::AppDataDir;
use keepass::config::DatabaseConfig;
use keepass::{Database, DatabaseKey};
use log::{debug, info};
use std::fs;
use std::fs::File;
use std::sync::{Arc, Mutex};
use tauri::Manager;
use xpassword_entity::{AppDefault, InitRequest};
use xpassword_error::CommonError;

pub mod accounts;

#[tauri::command]
pub fn app_default(app: tauri::AppHandle) -> Result<AppDefault, CommonError> {
    let app_data_dir = AppDataDir::new(
        app.path()
            .app_local_data_dir()
            .expect("could not resolve app local data path"),
    );

    Ok(AppDefault {
        kdbx_path: app_data_dir.accounts(),
    })
}

#[tauri::command]
pub fn init_app(app: tauri::AppHandle, request: InitRequest) -> Result<(), CommonError> {
    debug!("Initializing app request:{:?}", request);
    let state = app.state::<Arc<Mutex<AppState>>>();
    let mut app_state = state.lock().map_err(|_| CommonError::MutexLockFailed)?;
    let app_data_dir = AppDataDir::new(
        app.path()
            .app_local_data_dir()
            .expect("could not resolve app local data path"),
    );

    let mut db = Database::new(DatabaseConfig::default());
    db.meta.database_name = Some("Accounts Database".to_string());

    // Validate password strength
    utils::validate_password(&request.password)?;
    let kdbx_path = request.kdbx_path.clone();
    if !request.kdbx_path.exists() {
        db.save(
            &mut File::create(request.kdbx_path).expect("could not create kdbx file"),
            DatabaseKey::new().with_password(request.password.as_str()),
        )
        .expect("could not save kdbx");
    }

    let mut config = xpassword_config::Config::init(app_data_dir.config()).load();
    config
        .set_builder(config.builder().clone().set_kdbx_path(kdbx_path))
        .store();

    app_state.config = config;
    app_state.is_initialized = true;
    app_state.is_locked = false;
    app_state.runtime_timestamp = chrono::Local::now().timestamp() as u64;
    app_state.db = Some(db);
    app_state.master_password = Some(request.password);

    info!("app initialized");

    Ok(())
}
#[tauri::command]
pub fn launch_app(app: tauri::AppHandle) -> Result<(), CommonError> {
    let state = app.state::<Arc<Mutex<AppState>>>();
    let mut app_state = state.lock().map_err(|_| CommonError::MutexLockFailed)?;

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
    let _first_time = if !version_path.exists() {
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

    let cfg = xpassword_config::Config::init(app_data_dir.config()).load();

    let kdbx_path = cfg.builder().kdbx_path.clone();
    if !kdbx_path.exists() {
        return Err(CommonError::KdbxNotInitialized);
    }

    app_state.config = cfg;
    app_state.is_initialized = true;
    app_state.runtime_timestamp = chrono::Local::now().timestamp() as u64;
    app_state.is_locked = true;
    app_state.locked_timestamp = Some(app_state.runtime_timestamp);

    info!("app launch");

    Ok(())
}

#[tauri::command]
pub fn app_state(app: tauri::AppHandle) -> Result<AppState, CommonError> {
    let state = app.state::<Arc<Mutex<AppState>>>();
    let mut app_state = state.lock().map_err(|_| CommonError::MutexLockFailed)?;

    let settings = app_state.config.builder().settings.clone();
    let timeout = settings.auto_lock_timeout;
    let auto_lock_enabled = settings.auto_lock;

    if auto_lock_enabled && !app_state.is_locked {
        let now = chrono::Local::now().timestamp() as u64;
        if now.saturating_sub(app_state.runtime_timestamp) >= timeout {
            app_state.is_locked = true;
            app_state.locked_timestamp = Some(now);
            app_state.db = None;
            return Err(CommonError::AppIsLocked);
        }
    }

    Ok(app_state.clone())
}

#[tauri::command]
pub fn unlock_with_password(app: tauri::AppHandle, password: String) -> Result<(), CommonError> {
    if password.is_empty() {
        return Err(CommonError::RequestError("password is empty".to_string()));
    }
    
    let state = app.state::<Arc<Mutex<AppState>>>();
    let mut app_state = state.lock().map_err(|_| CommonError::MutexLockFailed)?;
    
    let kdbx_path = app_state.config.builder().kdbx_path.clone();
    
    // Try to open the database with the provided password
    if !kdbx_path.exists() {
        return Err(CommonError::KdbxNotInitialized);
    }
    
    let mut file = File::open(&kdbx_path)
        .map_err(|e| CommonError::UnexpectedError(anyhow::anyhow!("Failed to open KDBX file: {}", e)))?;
    
    let db = Database::open(
        &mut file,
        DatabaseKey::new().with_password(&password),
    )
    .map_err(|_| CommonError::InvalidPassword)?;
    
    app_state.db = Some(db);
    app_state.master_password = Some(password);
    app_state.is_locked = false;
    app_state.locked_timestamp = None;
    app_state.runtime_timestamp = chrono::Local::now().timestamp() as u64;
    
    Ok(())
}

#[tauri::command]
pub fn unlock_with_biometric(_app: tauri::AppHandle) {}

#[tauri::command]
pub fn lock(app: tauri::AppHandle) -> Result<(), String> {
    let state = app.state::<Arc<Mutex<AppState>>>();
    let mut state = state.lock().map_err(|_| CommonError::MutexLockFailed)?;
    state.is_locked = true;
    state.locked_timestamp = Some(chrono::Local::now().timestamp() as u64);
    state.db = None;
    state.master_password = None;
    Ok(())
}

#[tauri::command]
pub fn export_backup(_app: tauri::AppHandle, _password: String) {}

#[tauri::command]
pub fn import_backup(_app: tauri::AppHandle, _backup: Vec<u8>, _password: String) {}

#[tauri::command]
pub fn quit_app(app: tauri::AppHandle) {
    app.exit(0);
}
