use crate::state::AppState;
use crate::utils::app_data_dir::AppDataDir;
use keepass::config::DatabaseConfig;
use keepass::{Database, DatabaseKey, Entry, Group};
use log::{debug, error, info};
use std::fs;
use std::fs::File;
use std::sync::{Arc, Mutex};
use tauri::Manager;
use xauthenticator_entity::{AppDefault, InitRequest, CreateAccountRequest, UpdateAccountRequest, PagedResponse, PageParam};
use xauthenticator_error::CommonError;
use base64::{Engine as _, engine::general_purpose};

/// Validates password strength requirements
fn validate_password(password: &str) -> Result<(), CommonError> {
    if password.len() < 12 {
        return Err(CommonError::RequestError(
            "password must be at least 12 characters".to_string(),
        ));
    }
    
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(CommonError::RequestError(
            "password must contain at least one uppercase letter".to_string(),
        ));
    }
    
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(CommonError::RequestError(
            "password must contain at least one lowercase letter".to_string(),
        ));
    }
    
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(CommonError::RequestError(
            "password must contain at least one digit".to_string(),
        ));
    }
    
    // Check for special characters (non-alphanumeric, excluding whitespace)
    if !password.chars().any(|c| !c.is_alphanumeric() && !c.is_whitespace()) {
        return Err(CommonError::RequestError(
            "password must contain at least one special character".to_string(),
        ));
    }
    
    Ok(())
}

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
    let mut app_state = state.lock().unwrap();
    let app_data_dir = AppDataDir::new(
        app.path()
            .app_local_data_dir()
            .expect("could not resolve app local data path"),
    );

    let mut db = Database::new(DatabaseConfig::default());
    db.meta.database_name = Some("Accounts Database".to_string());

    // Validate password strength
    validate_password(&request.password)?;
    let kdbx_path = request.kdbx_path.clone();
    if !request.kdbx_path.exists() {
        db.save(
            &mut File::create(request.kdbx_path).expect("could not create kdbx file"),
            DatabaseKey::new().with_password(request.password.as_str()),
        )
        .expect("could not save kdbx");
    }

    let mut config = xauthenticator_config::Config::init(app_data_dir.config()).load();
    config
        .set_builder(config.builder().clone().set_kdbx_path(kdbx_path))
        .store();

    app_state.config = config;
    app_state.is_initialized = true;
    app_state.is_locked = false;
    app_state.runtime_timestamp = chrono::Local::now().timestamp() as u64;

    info!("Initializing app");

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

    let cfg = xauthenticator_config::Config::init(app_data_dir.config()).load();

    let kdbx_path = cfg.builder().kdbx_path.clone();
    if !kdbx_path.exists() {
        return Err(CommonError::KdbxNotInitialized);
    }

    let settings = cfg.builder().settings.clone();

    app_state.config = cfg;

    app_state.is_initialized = true;

    if settings.auto_lock && !app_state.is_locked {
        let now = chrono::Local::now().timestamp() as u64;
        if now.saturating_sub(app_state.runtime_timestamp) >= settings.auto_lock_timeout {
            app_state.is_locked = true;
            app_state.locked_timestamp = Some(now);
            return Err(CommonError::AppIsLocked);
        }
    }

    info!("app initialized");

    Ok(())
}

#[tauri::command]
pub fn app_state(app: tauri::AppHandle) -> Result<AppState, CommonError> {
    let state = app.state::<Arc<Mutex<AppState>>>();
    let mut app_state = state.lock().unwrap();

    let settings = app_state.config.builder().settings.clone();
    let timeout = settings.auto_lock_timeout;
    let auto_lock_enabled = settings.auto_lock;

    if auto_lock_enabled && !app_state.is_locked {
        let now = chrono::Local::now().timestamp() as u64;
        if now.saturating_sub(app_state.runtime_timestamp) >= timeout {
            app_state.is_locked = true;
            app_state.locked_timestamp = Some(now);
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
    let app_data_dir = AppDataDir::new(
        app.path()
            .app_local_data_dir()
            .expect("could not resolve app local data path"),
    );

    let state = app.state::<Arc<Mutex<AppState>>>();
    let mut state = state.lock().unwrap();
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
    Ok(())
}

/// Helper function to open the KDBX database with password
fn open_database(kdbx_path: &std::path::PathBuf, password: &str) -> Result<Database, CommonError> {
    if !kdbx_path.exists() {
        return Err(CommonError::KdbxNotInitialized);
    }
    
    let mut file = File::open(kdbx_path)
        .map_err(|e| CommonError::RequestError(format!("Failed to open database: {}", e)))?;
    
    let db = Database::open(&mut file, DatabaseKey::new().with_password(password))
        .map_err(|_| CommonError::InvalidPassword)?;
    
    Ok(db)
}

/// Helper function to save the KDBX database
fn save_database(db: &Database, kdbx_path: &std::path::PathBuf, password: &str) -> Result<(), CommonError> {
    let mut file = File::create(kdbx_path)
        .map_err(|e| CommonError::RequestError(format!("Failed to create database file: {}", e)))?;
    
    db.save(&mut file, DatabaseKey::new().with_password(password))
        .map_err(|e| CommonError::RequestError(format!("Failed to save database: {}", e)))?;
    
    Ok(())
}

/// Helper function to convert Entry to response Account
fn entry_to_account(entry: &Entry) -> xauthenticator_entity::response::Account {
    let id = entry.get_uuid().map(|uuid| uuid.to_string()).unwrap_or_default();
    let issuer = entry.get_title().unwrap_or("").to_string();
    let label = entry.get_username().unwrap_or("").to_string();
    let note = entry.get("Notes").map(|s| s.to_string());
    
    // Parse OTP fields from entry
    let otp_type = entry.get("otp_type").unwrap_or("TOTP").to_string();
    let algorithm = entry.get("otp_algorithm").unwrap_or("SHA1").to_string();
    let digits = entry.get("otp_digits").and_then(|s| s.parse::<i32>().ok()).unwrap_or(6);
    let period = entry.get("otp_period").and_then(|s| s.parse::<i32>().ok());
    let counter = entry.get("otp_counter").and_then(|s| s.parse::<i32>().ok());
    
    // Get secret (encrypted in KDBX by default)
    let secret_cipher = entry.get_password()
        .map(|s| s.as_bytes().to_vec())
        .unwrap_or_default();
    
    // Get icon
    let icon = entry.get_custom_icon_uuid().map(|_uuid| Vec::new());
    
    // Get timestamps
    let created_at = entry.times.get_creation_time().map(|t| t.timestamp());
    let updated_at = entry.times.get_last_modification().map(|t| t.timestamp());
    
    xauthenticator_entity::response::Account {
        id,
        issuer,
        label,
        r#type: otp_type,
        algorithm,
        digits,
        period,
        counter,
        secret_cipher,
        icon,
        note,
        created_at,
        updated_at,
    }
}

#[tauri::command]
pub async fn list_accounts(
    app: tauri::AppHandle,
    page_param: PageParam,
    password: String,
) -> Result<PagedResponse<xauthenticator_entity::response::Account>, CommonError> {
    // Gate: must be unlocked
    let (kdbx_path, is_locked) = {
        let state = app.state::<Arc<Mutex<AppState>>>();
        let state = state.lock().unwrap();
        (state.config.builder().kdbx_path.clone(), state.is_locked)
    };
    
    if is_locked {
        return Err(CommonError::AppIsLocked);
    }
    
    // Open database
    let db = open_database(&kdbx_path, &password)?;
    
    // Collect all entries from all groups
    let mut all_accounts: Vec<xauthenticator_entity::response::Account> = Vec::new();
    
    fn collect_entries(group: &Group, accounts: &mut Vec<xauthenticator_entity::response::Account>) {
        for entry in &group.entries {
            accounts.push(entry_to_account(entry));
        }
        for child_group in &group.children {
            collect_entries(child_group, accounts);
        }
    }
    
    collect_entries(&db.root, &mut all_accounts);
    
    let total = all_accounts.len() as u32;
    let start = ((page_param.current - 1) * page_param.size) as usize;
    let end = std::cmp::min(start + page_param.size as usize, all_accounts.len());
    
    let data = if start < all_accounts.len() {
        all_accounts[start..end].to_vec()
    } else {
        Vec::new()
    };
    
    Ok(PagedResponse {
        data,
        total,
        current: page_param.current,
        size: page_param.size,
    })
}

#[tauri::command]
pub fn add_account(
    app: tauri::AppHandle,
    request: CreateAccountRequest,
    password: String,
) -> Result<String, CommonError> {
    // Validate request
    if request.issuer.trim().is_empty() {
        return Err(CommonError::RequestError("issuer is required".to_string()));
    }
    if request.secret.trim().is_empty() {
        return Err(CommonError::RequestError("secret is required".to_string()));
    }
    
    info!("add_account: issuer={}, label={}", request.issuer, request.label);
    
    // Gate: must be unlocked
    let (kdbx_path, is_locked) = {
        let state = app.state::<Arc<Mutex<AppState>>>();
        let state = state.lock().unwrap();
        (state.config.builder().kdbx_path.clone(), state.is_locked)
    };
    
    if is_locked {
        return Err(CommonError::AppIsLocked);
    }
    
    // Open database
    let mut db = open_database(&kdbx_path, &password)?;
    
    // Create new entry
    let mut entry = Entry::default();
    let entry_uuid = uuid::Uuid::new_v7();
    entry.set_uuid(keepass::db::entry::Uuid::from_bytes(entry_uuid.as_bytes()));
    entry.set_title(&request.issuer);
    entry.set_username(&request.label);
    entry.set_password(&request.secret);
    
    // Set OTP-specific fields
    entry.add_tag("otp_type", &request.r#type);
    entry.add_tag("otp_algorithm", &request.algorithm);
    entry.add_tag("otp_digits", &request.digits.to_string());
    
    if let Some(period) = request.period {
        entry.add_tag("otp_period", &period.to_string());
    }
    if let Some(counter) = request.counter {
        entry.add_tag("otp_counter", &counter.to_string());
    }
    if let Some(note) = request.note {
        entry.add_tag("Notes", &note);
    }
    
    // Add entry to root group
    db.root.add_entry(entry);
    
    // Save database
    save_database(&db, &kdbx_path, &password)?;
    
    info!("add_account: successfully added account with id={}", entry_uuid);
    Ok(entry_uuid.to_string())
}

#[tauri::command]
pub fn update_account(
    app: tauri::AppHandle,
    request: UpdateAccountRequest,
    password: String,
) -> Result<(), CommonError> {
    if request.id.trim().is_empty() {
        return Err(CommonError::RequestError("account id is required".to_string()));
    }
    
    info!("update_account: id={}", request.id);
    
    // Gate: must be unlocked
    let (kdbx_path, is_locked) = {
        let state = app.state::<Arc<Mutex<AppState>>>();
        let state = state.lock().unwrap();
        (state.config.builder().kdbx_path.clone(), state.is_locked)
    };
    
    if is_locked {
        return Err(CommonError::AppIsLocked);
    }
    
    // Parse UUID
    let account_uuid = uuid::Uuid::parse_str(&request.id)
        .map_err(|_| CommonError::RequestError("invalid account id".to_string()))?;
    
    // Open database
    let mut db = open_database(&kdbx_path, &password)?;
    
    // Find and update entry
    let mut found = false;
    
    fn find_and_update_entry(
        group: &mut Group,
        uuid_bytes: &[u8],
        request: &UpdateAccountRequest,
    ) -> bool {
        for entry in &mut group.entries {
            if let Some(entry_uuid) = entry.get_uuid() {
                if entry_uuid.as_bytes() == uuid_bytes {
                    // Update fields
                    if let Some(ref issuer) = request.issuer {
                        entry.set_title(issuer);
                    }
                    if let Some(ref label) = request.label {
                        entry.set_username(label);
                    }
                    if let Some(ref note) = request.note {
                        entry.add_tag("Notes", note);
                    }
                    return true;
                }
            }
        }
        
        for child_group in &mut group.children {
            if find_and_update_entry(child_group, uuid_bytes, request) {
                return true;
            }
        }
        
        false
    }
    
    found = find_and_update_entry(&mut db.root, account_uuid.as_bytes(), &request);
    
    if !found {
        return Err(CommonError::RequestError("account not found".to_string()));
    }
    
    // Save database
    save_database(&db, &kdbx_path, &password)?;
    
    info!("update_account: successfully updated account with id={}", request.id);
    Ok(())
}

#[tauri::command]
pub async fn remove_account(
    app: tauri::AppHandle,
    account_id: String,
    password: String,
) -> Result<(), CommonError> {
    if account_id.trim().is_empty() {
        error!("remove_account: account_id is empty");
        return Err(CommonError::RequestError("account_id is required".to_string()));
    }
    
    info!("remove_account: account_id={}", account_id);
    
    // Gate: must be unlocked
    let (kdbx_path, is_locked) = {
        let state = app.state::<Arc<Mutex<AppState>>>();
        let state = state.lock().unwrap();
        (state.config.builder().kdbx_path.clone(), state.is_locked)
    };
    
    if is_locked {
        return Err(CommonError::AppIsLocked);
    }
    
    // Parse UUID
    let account_uuid = uuid::Uuid::parse_str(&account_id)
        .map_err(|_| CommonError::RequestError("invalid account id".to_string()))?;
    
    // Open database
    let mut db = open_database(&kdbx_path, &password)?;
    
    // Find and remove entry
    let mut found = false;
    
    fn find_and_remove_entry(group: &mut Group, uuid_bytes: &[u8]) -> bool {
        // Check if entry exists in this group
        if let Some(index) = group.entries.iter().position(|entry| {
            entry.get_uuid().map(|u| u.as_bytes() == uuid_bytes).unwrap_or(false)
        }) {
            group.entries.remove(index);
            return true;
        }
        
        // Recursively check child groups
        for child_group in &mut group.children {
            if find_and_remove_entry(child_group, uuid_bytes) {
                return true;
            }
        }
        
        false
    }
    
    found = find_and_remove_entry(&mut db.root, account_uuid.as_bytes());
    
    if !found {
        return Err(CommonError::RequestError("account not found".to_string()));
    }
    
    // Save database
    save_database(&db, &kdbx_path, &password)?;
    
    info!("remove_account: successfully removed account with id={}", account_id);
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
        if state.is_locked {
            return Err(CommonError::AppIsLocked);
        }
    }
    // Implementation of code generation would go here
    Err(CommonError::RequestError("not implemented".to_string()))
}

#[tauri::command]
pub fn quit_app(app: tauri::AppHandle) {
    app.exit(0);
}
