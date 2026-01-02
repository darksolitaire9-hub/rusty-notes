use sqlx::{SqlitePool, Row};
use std::path::PathBuf;
use std::fs;
use crate::db::schema::{Note, NoteWithAttachments};

pub struct DbStorage {
    pool: SqlitePool,
    attachments_dir: PathBuf,
}

impl DbStorage {
    pub fn new(pool: SqlitePool, app_data_dir: PathBuf) -> Self {
        let attachments_dir = app_data_dir.join("attachments");
        fs::create_dir_all(&attachments_dir).ok();
        
        Self {
            pool,
            attachments_dir,
        }
    }
    
    pub async fn create_note(&self, note: Note) -> Result<Note, String> {
        sqlx::query(
            "INSERT INTO notes (id, title, body, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&note.id)
        .bind(&note.title)
        .bind(&note.body)
        .bind(note.created_at)
        .bind(note.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to create note: {}", e))?;
        
        Ok(note)
    }
    
    pub async fn get_note(&self, id: &str) -> Result<NoteWithAttachments, String> {
        let row = sqlx::query(
            "SELECT id, title, body, created_at, updated_at 
             FROM notes 
             WHERE id = ? AND is_deleted = 0"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("Note not found: {}", e))?;
        
        let note = Note {
            id: row.get("id"),
            title: row.get("title"),
            body: row.get("body"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };
        
        Ok(NoteWithAttachments { 
            note, 
            attachments: vec![] // TODO: fetch attachments later
        })
    }
    
    pub async fn list_notes(&self) -> Result<Vec<Note>, String> {
        let rows = sqlx::query(
            "SELECT id, title, body, created_at, updated_at 
             FROM notes 
             WHERE is_deleted = 0 
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
        }).collect();
        
        Ok(notes)
    }
    
    pub async fn update_note(&self, note: Note) -> Result<Note, String> {
        sqlx::query(
            "UPDATE notes 
             SET title = ?, body = ?, updated_at = ? 
             WHERE id = ?"
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
    
    pub async fn delete_note(&self, id: &str) -> Result<(), String> {
        // Delete from database
        sqlx::query("DELETE FROM notes WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete note: {}", e))?;
        
        // Delete note's attachment folder
        let note_dir = self.attachments_dir.join(id);
        let _ = fs::remove_dir_all(note_dir);
        
        Ok(())
    }
    
    pub async fn search_notes(&self, query: &str) -> Result<Vec<Note>, String> {
        let search_pattern = format!("%{}%", query);
        
        let rows = sqlx::query(
            "SELECT id, title, body, created_at, updated_at 
             FROM notes 
             WHERE is_deleted = 0 
               AND (title LIKE ? OR body LIKE ?)
             ORDER BY updated_at DESC"
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to search notes: {}", e))?;
        
        let notes = rows.iter().map(|row| Note {
            id: row.get("id"),
            title: row.get("title"),
            body: row.get("body"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }).collect();
        
        Ok(notes)
    }
}
