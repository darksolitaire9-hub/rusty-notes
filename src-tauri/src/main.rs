#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod models;
mod services;
mod settings;
mod shortcuts;

use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            // Get app data directory
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");

            // Ensure directory exists
            std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");

            // Load/init settings
            let settings = crate::settings::load_or_init(
                app.path()
                    .app_config_dir()
                    .expect("Failed to get app config dir"),
            )
            .expect("Failed to load settings");

            // ✅ No manual scoping needed here either
            println!("✓ Settings loaded: {}", settings.notes_folder);

            // Register settings state (using tokio Mutex for async commands)
            let settings_state = Arc::new(Mutex::new(settings));
            app.manage(settings_state);

            // Initialize database
            let pool = tauri::async_runtime::block_on(async {
                db::init_database(&app_data_dir)
                    .await
                    .expect("Failed to initialize database")
            });

            // Create storage service
            let storage = Arc::new(Mutex::new(services::db_storage::DbStorage::new(
                pool,
                app_data_dir,
            )));

            // Store in app state
            app.manage(storage);

            // ✅ Register all shortcuts
            if let Err(e) = shortcuts::register_all(&app.handle()) {
                eprintln!("❌ Failed to register shortcuts: {}", e);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // note commands
            commands::notes::create_note,
            commands::notes::get_note,
            commands::notes::list_notes,
            commands::notes::update_note,
            commands::notes::delete_note,
            commands::notes::search_notes,
            // settings commands
            commands::settings_commands::get_settings,
            commands::settings_commands::complete_onboarding,
            commands::settings_commands::update_settings,
            // ✅ Remove restore_notes_directory_scope - not needed
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
