// src-tauri/src/services/permanent_delete.rs
use std::path::Path;
use std::fs;
use crate::db::schema::NoteWithAttachments;

pub struct PermanentDelete;

impl PermanentDelete {
    /// Permanently delete note and attachments (no recovery)
    pub async fn delete(
        note: &NoteWithAttachments,
    ) -> Result<(), String> {
        let note_path = Path::new(&note.note.file_path);

        // Delete note file
        if note_path.exists() {
            fs::remove_file(&note_path)
                .map_err(|e| format!("Failed to delete note file: {}", e))?;
        }

        // Delete attachments
        for attachment in &note.attachments {
            let attachment_path = Path::new(&attachment.file_path);
            if attachment_path.exists() {
                fs::remove_file(attachment_path)
                    .map_err(|e| format!("Failed to delete attachment: {}", e))?;
            }
        }

        Ok(())
    }
}
