use rand_core::TryRngCore;
use serde::Serialize;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tauri::Manager;

#[derive(Serialize)]
pub struct InitResult {
    data_dir: String,
    db_path: String,
    salt_path: String,
}

#[tauri::command]
pub fn init_app(app: tauri::AppHandle) -> Result<InitResult, String> {
    // Resolve app local data directory
    let data_dir: PathBuf = app
        .path()
        .app_local_data_dir()
        .expect("could not resolve app local data path");

    // Ensure the data directory exists
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).map_err(|e| format!("failed to create data dir: {}", e))?;
    }

    // Ensure salt.txt exists with random bytes
    let salt_path = data_dir.join("salt.txt");
    if !salt_path.exists() {
        // 16 bytes random salt; Argon2 accepts arbitrary-length salts
        let mut salt = [0u8; 16];
        rand_core::OsRng
            .try_fill_bytes(&mut salt)
            .expect("failed to generate random salt");
        let mut file = fs::File::create(&salt_path)
            .map_err(|e| format!("failed to create salt.txt: {}", e))?;
        file.write_all(&salt)
            .map_err(|e| format!("failed to write salt.txt: {}", e))?;
    }

    // Ensure a local SQLite database file exists
    let db_path = data_dir.join("xauthenticator.db");
    if !db_path.exists() {
        fs::File::create(&db_path).map_err(|e| format!("failed to create database file: {}", e))?;
    }

    Ok(InitResult {
        data_dir: data_dir.to_string_lossy().to_string(),
        db_path: db_path.to_string_lossy().to_string(),
        salt_path: salt_path.to_string_lossy().to_string(),
    })
}

#[tauri::command]
pub fn unlock_with_password(password: String) {
    println!("unlock with password: {}", password);
}

#[tauri::command]
pub fn unlock_with_biometric() {}

#[tauri::command]
pub fn lock() {}

#[tauri::command]
pub fn list_accounts() {}

#[tauri::command]
pub fn add_account(auth_url: String) {}

#[tauri::command]
pub fn remove_account(account_id: uuid::Uuid) {}

#[tauri::command]
pub fn export_backup(password: String) -> Vec<u8> {
    Vec::new()
}

#[tauri::command]
pub fn import_backup(backup: Vec<u8>, password: String) {}

#[tauri::command]
pub fn get_code(account_id: uuid::Uuid) {}

#[tauri::command]
pub fn health_check() {}
