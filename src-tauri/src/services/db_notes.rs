use sqlx::{SqlitePool, Row};
use crate::db::schema::Note;

/// Handles CRUD operations for notes table only.
pub struct NoteStorage {
    pool: SqlitePool,
}

impl NoteStorage {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Creates new note with file_path for filesystem ops.
    pub async fn create(&self, note: Note) -> Result<Note, String> {
        sqlx::query(
            "INSERT INTO notes (id, title, body, created_at, updated_at, file_path) 
             VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&note.id)
        .bind(&note.title)
        .bind(&note.body)
        .bind(note.created_at)
        .bind(note.updated_at)
        .bind(&note.file_path)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to create note: {}", e))?;

        Ok(note)
    }

    /// Fetches single active note.
    pub async fn get(&self, id: &str) -> Result<Note, String> {
        let row = sqlx::query(
            "SELECT id, title, body, created_at, updated_at, file_path 
             FROM notes WHERE id = ? AND is_deleted = 0"
        )
        .bind(id)
        .fetch_optional(&self.pool)  // Use optional to distinguish not found
        .await
        .map_err(|e| format!("Database error: {}", e))?;

        row.map(|r| Note {
            id: r.get("id"),
            title: r.get("title"),
            body: r.get("body"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
            file_path: r.get("file_path"),
        }).ok_or_else(|| "Note not found".to_string())
    }

    /// Lists all active notes, newest first.
    pub async fn list(&self) -> Result<Vec<Note>, String> {
        let rows = sqlx::query(
            "SELECT id, title, body, created_at, updated_at, file_path 
             FROM notes WHERE is_deleted = 0 
             ORDER BY updated_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to list notes: {}", e))?;

        let notes = rows.iter().map(|row| Note {
            id: row.get("id"),
            title: row.get("title"),
            body: row.get("body"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            file_path: row.get("file_path"),
        }).collect();

        Ok(notes)
    }

    /// Updates title/body/updated_at (preserves file_path).
    pub async fn update(&self, note: Note) -> Result<Note, String> {
        sqlx::query(
            "UPDATE notes SET title = ?, body = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&note.title)
        .bind(&note.body)
        .bind(note.updated_at)
        .bind(&note.id)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to update note: {}", e))?;

        Ok(note)
    }

    /// Hard deletes note (after filesystem ops).
    pub async fn delete(&self, id: &str) -> Result<(), String> {
        sqlx::query("DELETE FROM notes WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete note: {}", e))?;

        Ok(())
    }

    /// Full-text search on active notes.
    pub async fn search(&self, query: &str) -> Result<Vec<Note>, String> {
        let pattern = format!("%{}%", query);
        let rows = sqlx::query(
            "SELECT id, title, body, created_at, updated_at, file_path 
             FROM notes 
             WHERE is_deleted = 0 AND (title LIKE ? OR body LIKE ?)
             ORDER BY updated_at DESC"
        )
        .bind(&pattern)
        .bind(&pattern)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Search failed: {}", e))?;

        let notes = rows.iter().map(|row| Note {
            id: row.get("id"),
            title: row.get("title"),
            body: row.get("body"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            file_path: row.get("file_path"),
        }).collect();

        Ok(notes)
    }
}
