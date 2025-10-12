use log::info;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_cli::cli;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{App, Manager};

#[derive(Default)]
pub struct AppState {
    db: Option<DatabaseConnection>,
}

pub fn init_app(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    app.manage(Arc::new(Mutex::new(AppState::default())));
    let app_handle = app.handle();
    let current_version = format!("v{}", app.config().version.clone().unwrap());
    info!("app config version: {:?}", current_version);
    let app_data_dir: PathBuf = app
        .path()
        .app_local_data_dir()
        .expect("could not resolve app local data path");

    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir).expect("failed to create app data directory");
    }

    let version_path = app_data_dir.join("version.txt");
    let needs_initialization = if !version_path.exists() {
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
        if stored_version.trim() != current_version.as_str() {
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
        } else {
            false
        }
    };

    let db_path = app_data_dir.join(format!(
        "{}.db",
        app.config().identifier.to_lowercase().clone()
    ));
    if !db_path.exists() {
        info!("database file not found, creating new file");
        fs::File::create(&db_path).expect("failed to create database file");
        info!("created database file: {}", db_path.to_str().unwrap());
    }

    tauri::async_runtime::block_on(async move {
        let db: DatabaseConnection =
            Database::connect(format!("sqlite:{}", db_path.to_str().unwrap()))
                .await
                .expect("failed to connect to database");

        if needs_initialization {
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

        let state: tauri::State<'_, Arc<Mutex<AppState>>> = app_handle.state();
        let mut app_state = state.lock().unwrap();
        app_state.db = Some(db);
    });

    if needs_initialization {
        info!("performing initialization...");

        // TODO: 在这里添加具体的初始化代码
        // 例如:
        // - 初始化数据库
        // - 创建必要的配置文件
        // - 设置默认设置
        // - 迁移旧数据等

        info!("initialization completed");
    }

    Ok(())
}
