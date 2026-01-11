// src-tauri/src/services/delete_service.rs
use crate::db::schema::NoteWithAttachments;
use crate::services::{TrashManager, PermanentDelete};
use crate::settings::model::{Settings, DeleteBehavior};

pub struct DeleteService;

impl DeleteService {
    pub async fn delete_note(
        notes_folder: &str,
        note: &NoteWithAttachments,
        settings: &Settings,
    ) -> Result<(), String> {
        match settings.delete_behavior {
            DeleteBehavior::MoveToTrash => {
                TrashManager::move_to_trash(notes_folder, note).await
            }
            DeleteBehavior::Permanent => {
                PermanentDelete::delete(note).await
            }
        }
    }
}
