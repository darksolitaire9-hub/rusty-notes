use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::migrate::MigrateDatabase;
use sqlx::Sqlite;
use std::path::PathBuf;

pub mod schema;

pub async fn init_database(app_data_dir: &PathBuf) -> Result<SqlitePool, String> {
    let db_path = app_data_dir.join("rusty-notes.db");
    let db_url = format!("sqlite:{}", db_path.display());
    
    // Create database if it doesn't exist
    if !Sqlite::database_exists(&db_url)
        .await
        .unwrap_or(false)
    {
        Sqlite::create_database(&db_url)
            .await
            .map_err(|e| format!("Failed to create database: {}", e))?;
    }
    
    // Connect to database
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;
    
    // Run migrations
    run_migrations(&pool).await?;
    
    Ok(pool)
}

async fn run_migrations(pool: &SqlitePool) -> Result<(), String> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS notes (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            body TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            is_deleted INTEGER DEFAULT 0
        );
        
        CREATE TABLE IF NOT EXISTS attachments (
            id TEXT PRIMARY KEY,
            note_id TEXT NOT NULL,
            type TEXT NOT NULL,
            file_name TEXT NOT NULL,
            file_path TEXT NOT NULL,
            mime_type TEXT,
            size_bytes INTEGER,
            created_at INTEGER NOT NULL,
            FOREIGN KEY(note_id) REFERENCES notes(id) ON DELETE CASCADE
        );
        
        CREATE INDEX IF NOT EXISTS idx_attachments_note_id 
        ON attachments(note_id);
        
        CREATE INDEX IF NOT EXISTS idx_notes_updated 
        ON notes(updated_at DESC);
        "#
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to run migrations: {}", e))?;
    
    Ok(())
}
