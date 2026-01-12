mod create;

use tauri::AppHandle;

/// Register all note-related shortcuts
pub fn register_all(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    create::register(app)?;
    
    println!("  ✓ Note shortcuts registered");
    Ok(())
}

/// Unregister all note shortcuts
pub fn unregister_all(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    create::unregister(app)?;
    
    Ok(())
}
