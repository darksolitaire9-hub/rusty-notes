// src-tauri/src/services/trash_manager.rs
use std::path::Path;
use std::fs;
use crate::db::schema::NoteWithAttachments;

pub struct TrashManager;

impl TrashManager {
    pub async fn move_to_trash(
        notes_folder: &str,
        note: &NoteWithAttachments,
    ) -> Result<(), String> {
        let trash_root = Path::new(notes_folder).join("trash");
        let deletion_date = chrono::Local::now().format("%Y-%m-%d").to_string();
        let trash_date_folder = trash_root.join(&deletion_date);

        fs::create_dir_all(&trash_date_folder)
            .map_err(|e| format!("Failed to create trash folder: {}", e))?;

        // Move note file
        let original_note_path = Path::new(&note.note.file_path);
        if original_note_path.exists() {
            let filename = original_note_path
                .file_name()
                .ok_or("Invalid note filename")?;
            let trash_note_path = trash_date_folder.join(filename);

            fs::rename(&original_note_path, &trash_note_path)
                .map_err(|e| format!("Failed to move note to trash: {}", e))?;
        }

        // Move attachments
        for attachment in &note.attachments {
            let attachment_path = Path::new(&attachment.file_path);
            if attachment_path.exists() {
                let filename = attachment_path
                    .file_name()
                    .ok_or("Invalid attachment filename")?;
                let trash_attachment = trash_date_folder.join(filename);

                fs::rename(&attachment_path, &trash_attachment)
                    .map_err(|e| format!("Failed to move attachment: {}", e))?;
            }
        }

        Ok(())
    }
}
