// Declare our modules
mod commands;  // Commands folder
mod models;    // Models folder
mod services;  // Services folder

// Main entry point for the Tauri app
#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    tauri::Builder::default()
        // Register all our commands so Svelte can call them
        .invoke_handler(tauri::generate_handler![
            commands::save_note,
            commands::load_note,
            commands::list_notes,
            commands::delete_note
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
