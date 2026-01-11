// src-tauri/src/services/note_files.rs
use tokio::fs;
use std::path::PathBuf;

/// Handles all note file operations
pub struct NoteFileStore {
    notes_folder: String,
}

impl NoteFileStore {
    pub fn new(notes_folder: String) -> Self {
        Self { notes_folder }
    }

    /// Ensures the notes folder exists on disk
    pub async fn ensure_folder_exists(&self) -> Result<(), String> {
        fs::create_dir_all(&self.notes_folder)
            .await
            .map_err(|e| format!("Failed to create notes folder: {}", e))
    }

    /// Builds file path for a note ID: {notes_folder}/{id}.html
    pub fn path_for_id(&self, id: &str) -> String {
        PathBuf::from(&self.notes_folder)
            .join(format!("{}.html", id))
            .to_string_lossy()
            .to_string()
    }

    /// Writes note body to file
    pub async fn write_note_file(&self, file_path: &str, body: &str) -> Result<(), String> {
        fs::write(file_path, body.as_bytes())
            .await
            .map_err(|e| format!("Failed to write note file: {}", e))
    }
}
