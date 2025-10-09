#[tauri::command]
pub fn init_app() {}

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
