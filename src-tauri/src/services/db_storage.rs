use sqlx::SqlitePool;
use std::path::PathBuf;
use crate::services::{NoteStorage, AttachmentStorage};
use crate::db::schema::{Note, NoteWithAttachments, Attachment};

/// Facade combining note + attachment storage.
/// Used by Tauri commands. Delegates to specialized storage modules.
pub struct DbStorage {
    notes: NoteStorage,
    attachments: AttachmentStorage,
    app_data_dir: PathBuf,  // For filesystem cleanup
}

impl DbStorage {
    /// Creates complete storage with note/attachment subsystems.
    pub fn new(pool: SqlitePool, app_data_dir: PathBuf) -> Self {
        let notes = NoteStorage::new(pool.clone());
        let attachments = AttachmentStorage::new(pool.clone());
        
        Self {
            notes,
            attachments,
            app_data_dir,
        }
    }

    /// Full note + all its attachments (used by get_note command).
    pub async fn get_note(&self, id: &str) -> Result<NoteWithAttachments, String> {
        let note = self.notes.get(id).await?;
        let attachments = self.attachments.list_for_note(id).await?;
        Ok(NoteWithAttachments { note, attachments })
    }

    /// Creates note (file_path populated by commands layer).
    pub async fn create_note(&self, note: Note) -> Result<Note, String> {
        self.notes.create(note).await
    }

    /// Lists all active notes.
    pub async fn list_notes(&self) -> Result<Vec<Note>, String> {
        self.notes.list().await
    }

    /// Updates note (preserves file_path).
    pub async fn update_note(&self, note: Note) -> Result<Note, String> {
        self.notes.update(note).await
    }

    /// Hard delete: attachments first, then note + filesystem cleanup.
    pub async fn delete_note(&self, id: &str) -> Result<(), String> {
        // Delete attachments DB records
        self.attachments.delete_for_note(id).await?;
        
        // Delete note DB record
        self.notes.delete(id).await?;
        
        // Cleanup filesystem (attachments dir + note file handled by DeleteService)
        let attachments_dir = self.app_data_dir.join("attachments").join(id);
        let _ = std::fs::remove_dir_all(attachments_dir);
        
        Ok(())
    }

    /// Full-text search on notes.
    pub async fn search_notes(&self, query: &str) -> Result<Vec<Note>, String> {
        self.notes.search(query).await
    }

    /// Creates attachment record (after file saved to disk).
    pub async fn create_attachment(&self, attachment: Attachment) -> Result<(), String> {
        self.attachments.create(attachment).await
    }
}
