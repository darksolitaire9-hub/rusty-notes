#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Declare our modules
mod db;
mod commands;
mod services;
mod models; // Keep for now in case we need it

use tauri::Manager;
use std::sync::Arc;
use tokio::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // Get app data directory
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");
            
            // Ensure directory exists
            std::fs::create_dir_all(&app_data_dir)
                .expect("Failed to create app data directory");
            
            // Initialize database (using tokio runtime)
            let pool = tauri::async_runtime::block_on(async {
                db::init_database(&app_data_dir)
                    .await
                    .expect("Failed to initialize database")
            });
            
            // Create storage service
            let storage = Arc::new(Mutex::new(
                services::db_storage::DbStorage::new(pool, app_data_dir),
            ));
            
            // Store in app state
            app.manage(storage);
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::notes::create_note,
            commands::notes::get_note,
            commands::notes::list_notes,
            commands::notes::update_note,
            commands::notes::delete_note,
            commands::notes::search_notes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
