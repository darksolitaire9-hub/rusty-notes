// Import our Note model and StorageService
use crate::models::Note;
use crate::services::StorageService;
use tauri::Manager; 
/// Tauri command: Save a note to disk
/// Called from Svelte: invoke('save_note', { id, title, body })
#[tauri::command]
pub fn save_note(
    id: String,
    title: String,
    body: String,
    app_handle: tauri::AppHandle
) -> Result<(), String> {
    // Get the app data directory path
    let app_dir = app_handle.path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    
    // Create storage service with that path
    let storage = StorageService::new(app_dir)?;
    
    // Create a Note struct from the data
    let note = Note { id, title, body };
    
    // Save it to disk using the storage service
    storage.save(&note)
}

/// Tauri command: Load a note from disk
/// Called from Svelte: invoke('load_note', { id })
/// Returns: [title, body]
#[tauri::command]
pub fn load_note(
    id: String,
    app_handle: tauri::AppHandle
) -> Result<(String, String), String> {
    // Get the app data directory
    let app_dir = app_handle.path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    
    // Create storage service
    let storage = StorageService::new(app_dir)?;
    
    // Load the note from disk
    let note = storage.load(&id)?;
    
    // Return title and body as a tuple
    Ok((note.title, note.body))
}

/// Tauri command: List all note IDs
/// Called from Svelte: invoke('list_notes')
/// Returns: ["abc-123", "def-456", ...]
#[tauri::command]
pub fn list_notes(app_handle: tauri::AppHandle) -> Result<Vec<String>, String> {
    // Get the app data directory
    let app_dir = app_handle.path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    
    // Create storage service
    let storage = StorageService::new(app_dir)?;
    
    // List all note IDs
    storage.list()
}

/// Tauri command: Delete a note from disk
/// Called from Svelte: invoke('delete_note', { id })
#[tauri::command]
pub fn delete_note(
    id: String,
    app_handle: tauri::AppHandle
) -> Result<(), String> {
    // Get the app data directory
    let app_dir = app_handle.path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    
    // Create storage service
    let storage = StorageService::new(app_dir)?;
    
    // Delete the note
    storage.delete(&id)
}
