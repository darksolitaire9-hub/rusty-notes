use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::migrate::MigrateDatabase;
use sqlx::Sqlite;
use std::path::PathBuf;

pub mod schema;

/// Initialize SQLite:
/// - If the DB file doesn't exist, create it.
/// - Connect and run simple CREATE TABLE IF NOT EXISTS migrations.
/// - Does NOT try to alter or migrate existing schemas.
pub async fn init_database(app_data_dir: &PathBuf) -> Result<SqlitePool, String> {
    let db_path = app_data_dir.join("rusty-notes.db");
    let db_url = format!("sqlite:{}", db_path.display());

    // Create database file if it doesn't exist
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

    // For a brand-new DB, this will create tables.
    // For an existing DB, this will NO-OP (tables already exist).
    run_migrations(&pool).await?;

    Ok(pool)
}

async fn run_migrations(pool: &SqlitePool) -> Result<(), String> {
    sqlx::query(
        r#"
        -- Notes table (new DBs get file_path from day one)
        CREATE TABLE IF NOT EXISTS notes (
            id          TEXT PRIMARY KEY,
            title       TEXT NOT NULL,
            body        TEXT NOT NULL,
            created_at  INTEGER NOT NULL,
            updated_at  INTEGER NOT NULL,
            file_path   TEXT NOT NULL,
            is_deleted  INTEGER DEFAULT 0
        );

        -- Attachments table (already had file_path)
        CREATE TABLE IF NOT EXISTS attachments (
            id          TEXT PRIMARY KEY,
            note_id     TEXT NOT NULL,
            attachment_type        TEXT NOT NULL,
            file_name   TEXT NOT NULL,
            file_path   TEXT NOT NULL,
            mime_type   TEXT,
            size_bytes  INTEGER,
            created_at  INTEGER NOT NULL,
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
