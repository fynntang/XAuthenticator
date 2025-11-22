use crate::state::AppState;
use keepass::db::{Entry, Node};
use std::sync::{Arc, Mutex};
use tauri::Manager;
use xauthenticator_entity::{CreateAccountRequest, UpdateAccountRequest};
use xauthenticator_error::CommonError;

/// List all groups from the database
#[tauri::command]
pub async fn list_groups(app: tauri::AppHandle) -> Result<Vec<keepass::db::Group>, CommonError> {
    let state = app.state::<Arc<Mutex<AppState>>>();
    let app_state = state.lock().map_err(|_| CommonError::MutexLockFailed)?;

    if app_state.is_locked {
        return Err(CommonError::AppIsLocked);
    }
    let db = app_state
        .db
        .as_ref()
        .ok_or(CommonError::KdbxNotInitialized)?;
    let root_node = &db.root;
    Ok(root_node
        .groups()
        .iter()
        .map(|g| keepass::db::Group {
            uuid: g.uuid.clone(),
            name: g.name.clone(),
            notes: g.notes.clone(),
            icon_id: g.icon_id.clone(),
            custom_icon_uuid: g.custom_icon_uuid.clone(),
            children: g.children.clone(),
            times: g.times.clone(),
            custom_data: g.custom_data.clone(),
            is_expanded: g.is_expanded.clone(),
            default_autotype_sequence: g.default_autotype_sequence.clone(),
            enable_autotype: g.enable_autotype.clone(),
            enable_searching: g.enable_searching.clone(),
            last_top_visible_entry: g.last_top_visible_entry.clone(),
        })
        .collect())
}

/// List all tags from the database
#[tauri::command]
pub async fn list_tags(app: tauri::AppHandle) -> Result<Vec<String>, CommonError> {
    let state = app.state::<Arc<Mutex<AppState>>>();
    let app_state = state.lock().map_err(|_| CommonError::MutexLockFailed)?;
    if app_state.is_locked {
        return Err(CommonError::AppIsLocked);
    }
    let db = app_state
        .db
        .as_ref()
        .ok_or(CommonError::KdbxNotInitialized)?;

    let root_node = &db.root;

    let mut tags: Vec<String> = vec![];

    for node in &root_node.children {
        if let Node::Entry(e) = node {
            tags.extend(e.tags.clone())
        }
    }
    tags.sort();
    tags.dedup();

    Ok(tags)
}

/// List all accounts from the database
#[tauri::command]
pub async fn list_accounts(app: tauri::AppHandle) -> Result<Vec<keepass::db::Entry>, CommonError> {
    let state = app.state::<Arc<Mutex<AppState>>>();
    let app_state = state.lock().map_err(|_| CommonError::MutexLockFailed)?;

    if app_state.is_locked {
        return Err(CommonError::AppIsLocked);
    }
    let db = app_state
        .db
        .as_ref()
        .ok_or(CommonError::KdbxNotInitialized)?;
    let root_node = &db.root;

    Ok(entries(&root_node.children))
}

fn entries(nodes: &Vec<Node>) -> Vec<Entry> {
    let mut result: Vec<Entry> = Vec::new();
    for node in nodes {
        if let Node::Group(g) = node {
            result.extend(entries(&g.children));
        }
        if let Node::Entry(e) = node {
            result.push(e.clone())
        }
    }
    result
}

/// Create a new account
#[tauri::command]
pub async fn create_account(
    app: tauri::AppHandle,
    request: CreateAccountRequest,
) -> Result<(), CommonError> {
    let state = app.state::<Arc<Mutex<AppState>>>();
    let app_state = state.lock().map_err(|_| CommonError::MutexLockFailed)?;

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
    let mut app_state = state.lock().map_err(|_| CommonError::MutexLockFailed)?;

    if app_state.is_locked {
        return Err(CommonError::AppIsLocked);
    }

    Ok(())
}

/// Delete an account
#[tauri::command]
pub async fn delete_account(app: tauri::AppHandle, account_id: String) -> Result<(), CommonError> {
    let state = app.state::<Arc<Mutex<AppState>>>();
    let mut app_state = state.lock().map_err(|_| CommonError::MutexLockFailed)?;

    if app_state.is_locked {
        return Err(CommonError::AppIsLocked);
    }

    Ok(())
}

/// Generate TOTP code for an account
#[tauri::command]
pub fn get_code(app: tauri::AppHandle, account_id: String) -> Result<String, CommonError> {
    let state = app.state::<Arc<Mutex<AppState>>>();
    let app_state = state.lock().map_err(|_| CommonError::MutexLockFailed)?;

    if app_state.is_locked {
        return Err(CommonError::AppIsLocked);
    }
    Ok("".to_string())
}
