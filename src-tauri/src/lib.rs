mod commands;

use log::{error, info, warn};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::Manager;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();
    #[cfg(desktop)]
    {
        builder = builder
            .plugin(tauri_plugin_window_state::Builder::new().build())
            .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
                let windows = app.webview_windows();
                for (name, window) in windows {
                    if name == "main" {
                        if let Err(e) = window.show() {
                            warn!("Failed to show main window: {}", e);
                        }
                        if let Err(e) = window.unminimize() {
                            warn!("Failed to unminimize main window: {}", e);
                        }
                        if let Err(e) = window.set_focus() {
                            warn!("Failed to focus main window: {}", e);
                        }
                        break;
                    }
                }
            }));
    }

    #[cfg(any(mobile, target_os = "android", target_os = "ios"))]
    {
        builder = builder
            .plugin(tauri_plugin_barcode_scanner::init())
            .plugin(tauri_plugin_biometric::init())
    }

    builder
        .plugin(
            tauri_plugin_log::Builder::new()
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .build(),
        )
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .setup(|app| {
            let windows = app.webview_windows();
            for (name, window) in &windows {
                if name != "main" {
                    window.hide().expect("failed to hide window");
                }
            }

            let open_i = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&open_i, &quit_i])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "open" => {
                        let app_window = app.get_webview_window("main").unwrap();
                        app_window.show().unwrap();
                        app_window.unminimize().unwrap();
                        app_window.set_focus().unwrap()
                    }
                    "quit" => {
                        println!("quit menu item was clicked");
                        app.exit(0);
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .build(app)?;

            let salt_path = app
                .path()
                .app_local_data_dir()
                .expect("could not resolve app local data path")
                .join("salt.txt");
            info!("salt_path: {:?}", salt_path);

            app.handle()
                .plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;
            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::Resized { .. } => {}
            tauri::WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                } else {
                    window.show().unwrap();
                    window.unminimize().unwrap();
                    window.set_focus().unwrap();
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            commands::init_app,
            commands::unlock_with_password,
            commands::unlock_with_biometric,
            commands::lock,
            commands::list_accounts,
            commands::add_account,
            commands::remove_account,
            commands::export_backup,
            commands::import_backup,
            commands::get_code,
            commands::health_check,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
