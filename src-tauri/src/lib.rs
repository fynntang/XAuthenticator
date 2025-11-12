mod commands;
mod constants;
mod initialize;
mod state;
mod utils;

use constants::webview_window_labels::WebviewWindowLabels;
use state::AppState;
use std::sync::{Arc, Mutex};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window(WebviewWindowLabels::Main.to_string().as_str())
                .expect("no main window")
                .set_focus();
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
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .manage(Arc::new(Mutex::new(AppState::default())))
        .setup(|_app| Ok(()))
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
            commands::app_default,
            commands::init_app,
            commands::launch_app,
            commands::app_state,
            commands::unlock_with_password,
            commands::unlock_with_biometric,
            commands::lock,
            commands::list_accounts,
            commands::add_account,
            commands::remove_account,
            commands::export_backup,
            commands::import_backup,
            commands::get_code,
            commands::quit_app,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
