use crate::state::AppState;
use std::sync::{Arc, Mutex};
use tauri::Manager;
use xauthenticator_entity::{CreateAccountRequest, UpdateAccountRequest};
use xauthenticator_error::CommonError;

/// List all accounts from the database
#[tauri::command]
pub async fn list_accounts(app: tauri::AppHandle) -> Result<keepass::db::Group, CommonError> {
    let state = app.state::<Arc<Mutex<AppState>>>();
    let app_state = state.lock().unwrap();

    if app_state.is_locked {
        return Err(CommonError::AppIsLocked);
    }
    let db = app_state
        .db
        .clone()
        .ok_or(CommonError::KdbxNotInitialized)?;

    Ok(db.root)
}

/// Create a new account
#[tauri::command]
pub async fn create_account(
    app: tauri::AppHandle,
    request: CreateAccountRequest,
) -> Result<(), CommonError> {
    let state = app.state::<Arc<Mutex<AppState>>>();
    let mut app_state = state.lock().unwrap();

    if app_state.is_locked {
        return Err(CommonError::AppIsLocked);
    }

    Ok(())
}

/// Update an existing account
#[tauri::command]
pub async fn update_account(
    app: tauri::AppHandle,
    request: UpdateAccountRequest,
) -> Result<(), CommonError> {
    let state = app.state::<Arc<Mutex<AppState>>>();
    let mut app_state = state.lock().unwrap();

    if app_state.is_locked {
        return Err(CommonError::AppIsLocked);
    }

    Ok(())
}

/// Delete an account
#[tauri::command]
pub async fn delete_account(app: tauri::AppHandle, account_id: String) -> Result<(), CommonError> {
    let state = app.state::<Arc<Mutex<AppState>>>();
    let mut app_state = state.lock().unwrap();

    if app_state.is_locked {
        return Err(CommonError::AppIsLocked);
    }

    Ok(())
}

/// Generate TOTP code for an account
#[tauri::command]
pub fn get_code(app: tauri::AppHandle, account_id: String) -> Result<String, CommonError> {
    let state = app.state::<Arc<Mutex<AppState>>>();
    let app_state = state.lock().unwrap();

    if app_state.is_locked {
        return Err(CommonError::AppIsLocked);
    }
    Ok("".to_string())
}
