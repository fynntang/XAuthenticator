use crate::state::AppState;
use crate::utils::app_data_dir::AppDataDir;
use crate::utils::check_file_exists::CheckFileExists;
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
    let parsed = match parse_otpauth(&auth_url) {
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

// Helpers for parsing otpauth URIs (minimal implementation)
#[derive(Debug, Clone)]
struct ParsedOtp {
    issuer: String,
    label: String,
    r#type: String,
    algorithm: String,
    digits: i32,
    period: Option<i32>,
    counter: Option<i32>,
    secret: Vec<u8>,
}

fn percent_decode(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hex = &input[i + 1..i + 3];
            if let Ok(val) = u8::from_str_radix(hex, 16) {
                result.push(val as char);
                i += 3;
                continue;
            }
        }
        let ch = if bytes[i] == b'+' {
            ' '
        } else {
            bytes[i] as char
        };
        result.push(ch);
        i += 1;
    }
    result
}

fn parse_otpauth(auth_url: &str) -> Option<ParsedOtp> {
    let lower = auth_url.to_lowercase();
    if !lower.starts_with("otpauth://") {
        return None;
    }
    let rest = &auth_url[10..];
    let slash_idx = rest.find('/')?;
    let typ_raw = &rest[..slash_idx];
    let remainder = &rest[slash_idx + 1..];

    let mut label_raw = remainder;
    let mut query = "";
    if let Some(qpos) = remainder.find('?') {
        label_raw = &remainder[..qpos];
        query = &remainder[qpos + 1..];
    }

    let label_decoded = percent_decode(label_raw);

    let mut issuer_from_label: Option<String> = None;
    let label_final: String;
    if let Some(pos) = label_decoded.find(':') {
        issuer_from_label = Some(label_decoded[..pos].trim().to_string());
        label_final = label_decoded[pos + 1..].trim().to_string();
    } else {
        label_final = label_decoded.trim().to_string();
    }

    // Parse query params
    let mut params: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    for pair in query.split('&') {
        if pair.is_empty() {
            continue;
        }
        let mut it = pair.splitn(2, '=');
        let k = it.next().unwrap_or("");
        let v = percent_decode(it.next().unwrap_or(""));
        params.insert(k.to_lowercase(), v);
    }

    let secret = params.get("secret")?.as_bytes().to_vec();
    let issuer = if let Some(iss) = params.get("issuer") {
        if iss.is_empty() {
            issuer_from_label.clone().unwrap_or_default()
        } else {
            iss.clone()
        }
    } else {
        issuer_from_label.clone().unwrap_or_default()
    };

    let algorithm = params
        .get("algorithm")
        .map(|s| s.to_uppercase())
        .unwrap_or_else(|| "SHA1".to_string());

    let digits = params
        .get("digits")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(6);

    let typ = if typ_raw.eq_ignore_ascii_case("hotp") {
        "HOTP".to_string()
    } else {
        // default to TOTP
        "TOTP".to_string()
    };

    let period = if typ == "TOTP" {
        Some(
            params
                .get("period")
                .and_then(|s| s.parse::<i32>().ok())
                .unwrap_or(30),
        )
    } else {
        None
    };

    let counter = if typ == "HOTP" {
        Some(
            params
                .get("counter")
                .and_then(|s| s.parse::<i32>().ok())
                .unwrap_or(0),
        )
    } else {
        None
    };

    Some(ParsedOtp {
        issuer,
        label: label_final,
        r#type: typ,
        algorithm,
        digits,
        period,
        counter,
        secret,
    })
}
