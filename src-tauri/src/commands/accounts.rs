use crate::state::AppState;
use keepass::db::{Entry, Node, Value};
use keepass::{Database, DatabaseKey};
use log::{error, info};
use std::fs::File;
use std::sync::{Arc, Mutex};
use tauri::Manager;
use xauthenticator_entity::{Account, CreateAccountRequest, UpdateAccountRequest};
use xauthenticator_error::CommonError;

/// Save the database to the KDBX file
fn save_database(
    db: &mut Database,
    kdbx_path: &std::path::PathBuf,
    password: &str,
) -> Result<(), CommonError> {
    let mut file = File::create(kdbx_path)
        .map_err(|e| CommonError::UnexpectedError(anyhow::anyhow!("Failed to create KDBX file: {}", e)))?;

    db.save(&mut file, DatabaseKey::new().with_password(password))
        .map_err(|e| {
            error!("Failed to save database: {:?}", e);
            CommonError::UnexpectedError(anyhow::anyhow!("Failed to save database: {}", e))
        })?;

    Ok(())
}

/// Convert a KDBX Entry to an Account
fn entry_to_account(entry: &Entry) -> Option<Account> {
    let id = entry.uuid.to_string();
    let name = entry.get_title().unwrap_or("").to_string();
    
    // Get TOTP configuration from custom fields
    let secret = entry.get("TOTP_SECRET")
        .and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) })?;

    let issuer = entry.get("TOTP_ISSUER").map(|s| s.to_string());
    let account_name = entry.get_username().map(|s| s.to_string());
    let algorithm = entry.get("TOTP_ALGORITHM").unwrap_or("SHA1").to_string();
    let digits: u32 = entry
        .get("TOTP_DIGITS")
        .and_then(|s| s.parse().ok())
        .unwrap_or(6);
    let period: u32 = entry
        .get("TOTP_PERIOD")
        .and_then(|s| s.parse().ok())
        .unwrap_or(30);
    let icon = entry.get("ICON").map(|s| s.to_string());
    
    // Get notes from the Notes field
    let notes = entry.get("Notes").map(|s| s.to_string());

    let created_at = entry.times.get_creation()
        .map(|t| t.and_utc().timestamp())
        .unwrap_or(0);
    let modified_at = entry.times.get_last_modification()
        .map(|t| t.and_utc().timestamp())
        .unwrap_or(0);

    Some(Account {
        id,
        name,
        issuer,
        account_name,
        secret,
        algorithm,
        digits,
        period,
        icon,
        notes,
        created_at,
        modified_at,
    })
}

/// Update an Entry from an Account
fn update_entry_from_account(entry: &mut Entry, account: &Account) {
    entry.fields.insert("Title".to_string(), Value::Unprotected(account.name.clone()));
    
    if let Some(ref account_name) = account.account_name {
        entry.fields.insert("UserName".to_string(), Value::Unprotected(account_name.clone()));
    }
    
    if let Some(ref notes) = account.notes {
        entry.fields.insert("Notes".to_string(), Value::Unprotected(notes.clone()));
    }

    entry.fields.insert("TOTP_SECRET".to_string(), Value::Unprotected(account.secret.clone()));
    entry.fields.insert("TOTP_ALGORITHM".to_string(), Value::Unprotected(account.algorithm.clone()));
    entry.fields.insert("TOTP_DIGITS".to_string(), Value::Unprotected(account.digits.to_string()));
    entry.fields.insert("TOTP_PERIOD".to_string(), Value::Unprotected(account.period.to_string()));
    
    if let Some(ref issuer) = account.issuer {
        entry.fields.insert("TOTP_ISSUER".to_string(), Value::Unprotected(issuer.clone()));
    }
    
    if let Some(ref icon) = account.icon {
        entry.fields.insert("ICON".to_string(), Value::Unprotected(icon.clone()));
    }
}

/// List all accounts from the database
#[tauri::command]
pub async fn list_accounts(app: tauri::AppHandle) -> Result<Vec<Account>, CommonError> {
    let state = app.state::<Arc<Mutex<AppState>>>();
    let app_state = state.lock().unwrap();
    
    if app_state.is_locked {
        return Err(CommonError::AppIsLocked);
    }

    if let Some(ref db) = app_state.db {
        let mut accounts = Vec::new();
        
        // Iterate through all groups and entries
        for node in &db.root.children {
            match node {
                Node::Group(group) => {
                    for entry_node in &group.children {
                        if let Node::Entry(entry) = entry_node {
                            if let Some(account) = entry_to_account(entry) {
                                accounts.push(account);
                            }
                        }
                    }
                }
                Node::Entry(entry) => {
                    if let Some(account) = entry_to_account(entry) {
                        accounts.push(account);
                    }
                }
            }
        }
        
        Ok(accounts)
    } else {
        Ok(Vec::new())
    }
}

/// Create a new account
#[tauri::command]
pub async fn create_account(
    app: tauri::AppHandle,
    request: CreateAccountRequest,
) -> Result<Account, CommonError> {
    let state = app.state::<Arc<Mutex<AppState>>>();
    let mut app_state = state.lock().unwrap();
    
    if app_state.is_locked {
        return Err(CommonError::AppIsLocked);
    }

    if request.secret.trim().is_empty() {
        return Err(CommonError::RequestError("Secret cannot be empty".to_string()));
    }

    let account = Account {
        id: uuid::Uuid::new_v4().to_string(),
        name: request.name,
        issuer: request.issuer,
        account_name: request.account_name,
        secret: request.secret,
        algorithm: request.algorithm.unwrap_or("SHA1".to_string()),
        digits: request.digits.unwrap_or(6),
        period: request.period.unwrap_or(30),
        icon: request.icon,
        notes: request.notes,
        created_at: chrono::Local::now().timestamp(),
        modified_at: chrono::Local::now().timestamp(),
    };

    // Add to database
    let kdbx_path = app_state.config.builder().kdbx_path.clone();
    let password = app_state.master_password.as_ref()
        .ok_or(CommonError::MasterKeyNotInitialized)?
        .clone();
    
    if let Some(ref mut db) = app_state.db {
        let mut entry = Entry::default();
        entry.uuid = uuid::Uuid::parse_str(&account.id).unwrap();
        update_entry_from_account(&mut entry, &account);
        
        // Add entry to root group
        db.root.add_child(Node::Entry(entry));
        
        // Save database
        save_database(db, &kdbx_path, &password)?;
        
        info!("Created account: {} ({})", account.name, account.id);
        Ok(account)
    } else {
        Err(CommonError::KdbxNotInitialized)
    }
}

/// Update an existing account
#[tauri::command]
pub async fn update_account(
    app: tauri::AppHandle,
    request: UpdateAccountRequest,
) -> Result<Account, CommonError> {
    let state = app.state::<Arc<Mutex<AppState>>>();
    let mut app_state = state.lock().unwrap();
    
    if app_state.is_locked {
        return Err(CommonError::AppIsLocked);
    }

    let account_id = uuid::Uuid::parse_str(&request.id)
        .map_err(|_| CommonError::RequestError("Invalid account ID".to_string()))?;

    let kdbx_path = app_state.config.builder().kdbx_path.clone();
    let password = app_state.master_password.as_ref()
        .ok_or(CommonError::MasterKeyNotInitialized)?
        .clone();

    if let Some(ref mut db) = app_state.db {
        let mut found_account: Option<Account> = None;
        
        // Search and update in all groups and root
        for node in &mut db.root.children {
            match node {
                Node::Group(group) => {
                    for entry_node in &mut group.children {
                        if let Node::Entry(entry) = entry_node {
                            if entry.uuid == account_id {
                                // Update the entry fields
                                if let Some(ref name) = request.name {
                                    entry.fields.insert("Title".to_string(), Value::Unprotected(name.clone()));
                                }
                                if let Some(ref account_name) = request.account_name {
                                    entry.fields.insert("UserName".to_string(), Value::Unprotected(account_name.clone()));
                                }
                                if let Some(ref notes) = request.notes {
                                    entry.fields.insert("Notes".to_string(), Value::Unprotected(notes.clone()));
                                }
                                if let Some(ref secret) = request.secret {
                                    entry.fields.insert("TOTP_SECRET".to_string(), Value::Unprotected(secret.clone()));
                                }
                                if let Some(ref algorithm) = request.algorithm {
                                    entry.fields.insert("TOTP_ALGORITHM".to_string(), Value::Unprotected(algorithm.clone()));
                                }
                                if let Some(digits) = request.digits {
                                    entry.fields.insert("TOTP_DIGITS".to_string(), Value::Unprotected(digits.to_string()));
                                }
                                if let Some(period) = request.period {
                                    entry.fields.insert("TOTP_PERIOD".to_string(), Value::Unprotected(period.to_string()));
                                }
                                if let Some(ref issuer) = request.issuer {
                                    entry.fields.insert("TOTP_ISSUER".to_string(), Value::Unprotected(issuer.clone()));
                                }
                                if let Some(ref icon) = request.icon {
                                    entry.fields.insert("ICON".to_string(), Value::Unprotected(icon.clone()));
                                }
                                
                                // Update modification time
                                entry.times.set_last_modification(chrono::Utc::now().naive_utc());
                                
                                found_account = entry_to_account(entry);
                                break;
                            }
                        }
                    }
                }
                Node::Entry(entry) => {
                    if entry.uuid == account_id {
                        if let Some(ref name) = request.name {
                            entry.fields.insert("Title".to_string(), Value::Unprotected(name.clone()));
                        }
                        if let Some(ref account_name) = request.account_name {
                            entry.fields.insert("UserName".to_string(), Value::Unprotected(account_name.clone()));
                        }
                        if let Some(ref notes) = request.notes {
                            entry.fields.insert("Notes".to_string(), Value::Unprotected(notes.clone()));
                        }
                        if let Some(ref secret) = request.secret {
                            entry.fields.insert("TOTP_SECRET".to_string(), Value::Unprotected(secret.clone()));
                        }
                        if let Some(ref algorithm) = request.algorithm {
                            entry.fields.insert("TOTP_ALGORITHM".to_string(), Value::Unprotected(algorithm.clone()));
                        }
                        if let Some(digits) = request.digits {
                            entry.fields.insert("TOTP_DIGITS".to_string(), Value::Unprotected(digits.to_string()));
                        }
                        if let Some(period) = request.period {
                            entry.fields.insert("TOTP_PERIOD".to_string(), Value::Unprotected(period.to_string()));
                        }
                        if let Some(ref issuer) = request.issuer {
                            entry.fields.insert("TOTP_ISSUER".to_string(), Value::Unprotected(issuer.clone()));
                        }
                        if let Some(ref icon) = request.icon {
                            entry.fields.insert("ICON".to_string(), Value::Unprotected(icon.clone()));
                        }
                        
                        entry.times.set_last_modification(chrono::Utc::now().naive_utc());
                        
                        found_account = entry_to_account(entry);
                        break;
                    }
                }
            }
            
            if found_account.is_some() {
                break;
            }
        }
        
        if let Some(account) = found_account {
            // Save database
            save_database(db, &kdbx_path, &password)?;
            
            info!("Updated account: {} ({})", account.name, account.id);
            Ok(account)
        } else {
            Err(CommonError::RequestError("Account not found".to_string()))
        }
    } else {
        Err(CommonError::KdbxNotInitialized)
    }
}

/// Delete an account
#[tauri::command]
pub async fn delete_account(
    app: tauri::AppHandle,
    account_id: String,
) -> Result<(), CommonError> {
    let state = app.state::<Arc<Mutex<AppState>>>();
    let mut app_state = state.lock().unwrap();
    
    if app_state.is_locked {
        return Err(CommonError::AppIsLocked);
    }

    let account_uuid = uuid::Uuid::parse_str(&account_id)
        .map_err(|_| CommonError::RequestError("Invalid account ID".to_string()))?;

    let kdbx_path = app_state.config.builder().kdbx_path.clone();
    let password = app_state.master_password.as_ref()
        .ok_or(CommonError::MasterKeyNotInitialized)?
        .clone();

    if let Some(ref mut db) = app_state.db {
        let mut found = false;
        
        // Search and delete from all groups and root
        for node in &mut db.root.children {
            if let Node::Group(group) = node {
                let initial_len = group.children.len();
                group.children.retain(|child_node| {
                    if let Node::Entry(entry) = child_node {
                        entry.uuid != account_uuid
                    } else {
                        true
                    }
                });
                if group.children.len() < initial_len {
                    found = true;
                    break;
                }
            }
        }
        
        // Also check root level
        if !found {
            let initial_len = db.root.children.len();
            db.root.children.retain(|node| {
                if let Node::Entry(entry) = node {
                    entry.uuid != account_uuid
                } else {
                    true
                }
            });
            found = db.root.children.len() < initial_len;
        }
        
        if found {
            // Save database
            save_database(db, &kdbx_path, &password)?;
            
            info!("Deleted account: {}", account_id);
            Ok(())
        } else {
            Err(CommonError::RequestError("Account not found".to_string()))
        }
    } else {
        Err(CommonError::KdbxNotInitialized)
    }
}

/// Generate TOTP code for an account
#[tauri::command]
pub fn get_code(app: tauri::AppHandle, account_id: String) -> Result<String, CommonError> {
    let state = app.state::<Arc<Mutex<AppState>>>();
    let app_state = state.lock().unwrap();
    
    if app_state.is_locked {
        return Err(CommonError::AppIsLocked);
    }

    let account_uuid = uuid::Uuid::parse_str(&account_id)
        .map_err(|_| CommonError::RequestError("Invalid account ID".to_string()))?;

    if let Some(ref db) = app_state.db {
        // Find the account
        for node in &db.root.children {
            match node {
                Node::Group(group) => {
                    for entry_node in &group.children {
                        if let Node::Entry(entry) = entry_node {
                            if entry.uuid == account_uuid {
                                if let Some(account) = entry_to_account(entry) {
                                    return generate_totp_code(&account);
                                }
                            }
                        }
                    }
                }
                Node::Entry(entry) => {
                    if entry.uuid == account_uuid {
                        if let Some(account) = entry_to_account(entry) {
                            return generate_totp_code(&account);
                        }
                    }
                }
            }
        }
        
        Err(CommonError::RequestError("Account not found".to_string()))
    } else {
        Err(CommonError::KdbxNotInitialized)
    }
}

fn generate_totp_code(account: &Account) -> Result<String, CommonError> {
    // Decode the base32 secret
    let secret_bytes = base32::decode(base32::Alphabet::Rfc4648 { padding: false }, &account.secret)
        .ok_or_else(|| CommonError::RequestError("Invalid secret format".to_string()))?;
    
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let code = match account.algorithm.as_str() {
        "SHA256" => totp_lite::totp_custom::<totp_lite::Sha256>(
            account.period as u64,
            account.digits,
            &secret_bytes,
            time,
        ),
        "SHA512" => totp_lite::totp_custom::<totp_lite::Sha512>(
            account.period as u64,
            account.digits,
            &secret_bytes,
            time,
        ),
        _ => totp_lite::totp_custom::<totp_lite::Sha1>(
            account.period as u64,
            account.digits,
            &secret_bytes,
            time,
        ),
    };
    
    Ok(format!("{:0width$}", code, width = account.digits as usize))
}
