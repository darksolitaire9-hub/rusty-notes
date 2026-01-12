pub mod notes;

use tauri::AppHandle;

pub fn register_all(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    println!("➡️ register_all() called");
    notes::register_all(app)?;
    Ok(())
}

pub fn unregister_all(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    notes::unregister_all(app)?;
    Ok(())
}
