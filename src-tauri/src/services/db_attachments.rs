use sqlx::{SqlitePool, Row};
use crate::db::schema::Attachment;

/// Manages attachment records only.
/// Links attachments to notes via note_id.
pub struct AttachmentStorage {
    pool: SqlitePool,
}

impl AttachmentStorage {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Creates attachment record after saving file to disk.
    pub async fn create(&self, attachment: Attachment) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO attachments (id, note_id, attachment_type, file_name, file_path, mime_type, size_bytes, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&attachment.id)
        .bind(&attachment.note_id)
        .bind(&attachment.attachment_type)
        .bind(&attachment.file_name)
        .bind(&attachment.file_path)
        .bind(&attachment.mime_type.as_deref())
        .bind(attachment.size_bytes)
        .bind(attachment.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to create attachment: {}", e))?;

        Ok(())
    }

    /// Lists all attachments for a note.
    pub async fn list_for_note(&self, note_id: &str) -> Result<Vec<Attachment>, String> {
        let rows = sqlx::query(
            "SELECT id, note_id, attachment_type, file_name, file_path, mime_type, size_bytes, created_at
             FROM attachments WHERE note_id = ?"
        )
        .bind(note_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to list attachments: {}", e))?;

        let attachments = rows.iter().map(|row| Attachment {
            id: row.get("id"),
            note_id: row.get("note_id"),
            attachment_type: row.get("attachment_type"),
            file_name: row.get("file_name"),
            file_path: row.get("file_path"),
            mime_type: row.get("mime_type"),
            size_bytes: row.get("size_bytes"),
            created_at: row.get("created_at"),
        }).collect();

        Ok(attachments)
    }

    /// Deletes all attachments for a note (called before note delete).
    pub async fn delete_for_note(&self, note_id: &str) -> Result<(), String> {
        sqlx::query("DELETE FROM attachments WHERE note_id = ?")
            .bind(note_id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete attachments: {}", e))?;

        Ok(())
    }
}
