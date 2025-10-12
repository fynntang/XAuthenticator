mod commands;
mod initialize;

use chrono::Datelike;
use log::{info, warn};
use std::sync::Mutex;
use tauri::menu::{
    AboutMetadata, AboutMetadataBuilder, IconMenuItem, Menu, MenuBuilder, MenuItem, NativeIcon,
    PredefinedMenuItem,
};
use tauri::tray::{MouseButton, TrayIconBuilder, TrayIconEvent};
use tauri::{Manager, Webview};
use tauri_plugin_opener::OpenerExt;

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
            initialize::init_app(app).expect("failed to initialize app");

            let windows = app.webview_windows();
            for (name, window) in &windows {
                if name != "main" {
                    window.hide().expect("failed to hide window");
                }
            }

            let menu = MenuBuilder::new(app)
                .text("open", "Open App")
                .text("official_website", "Official Website")
                .about_with_text(
                    "About",
                    Option::from(
                        AboutMetadataBuilder::new()
                            .name(Option::from(app.config().product_name.clone().unwrap()))
                            .short_version(Option::from(app.config().version.clone().unwrap()))
                            .authors(Option::from(vec!["XAuthenticator Team".to_string()]))
                            .website(Option::from(
                                "https://github.com/fynntang/XAuthenticator".to_string(),
                            ))
                            .copyright(Option::from(format!(
                                "Copyright © {} XAuthenticator Contributors",
                                chrono::Local::now().year()
                            )))
                            .license(Option::from("MIT License"))
                            .build(),
                    ),
                )
                .separator()
                .paste()
                .separator()
                .text("settings", "Settings")
                .quit_with_text("Quit App")
                .build()?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("XAuthenticator")
                .show_menu_on_left_click(false)
                .menu(&menu)
                .on_menu_event(|app_handle, event| match event.id.as_ref() {
                    "open" => {
                        let app_window = app_handle.get_webview_window("main").unwrap();
                        app_window.show().unwrap();
                        app_window.unminimize().unwrap();
                        app_window.set_focus().unwrap()
                    }
                    "official_website" => {
                        // app.opener().open_url("https://github.com/fynntang/XAuthenticator", None::<&str>).expect("failed to open url");
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .on_tray_icon_event(|icon, event| match event {
                    TrayIconEvent::Click {
                        id,
                        position,
                        rect,
                        button,
                        button_state,
                    } => {
                        info!(
                            "id: {:?} position: {:?} rect: {:?} button: {:?} button_state: {:?}",
                            id, position, rect, button, button_state
                        );
                        if button == MouseButton::Left {
                            let app_window = icon.app_handle().get_webview_window("main").unwrap();
                            app_window.show().unwrap();
                            app_window.unminimize().unwrap();
                            app_window.set_focus().unwrap();
                            return;
                        }
                    }
                    _ => {}
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
