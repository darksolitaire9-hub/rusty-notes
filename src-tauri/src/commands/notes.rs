use tauri::State;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::services::db_storage::DbStorage;
use crate::db::schema::{Note, NoteWithAttachments};
use chrono::Utc;

type StorageState = Arc<Mutex<DbStorage>>;

#[tauri::command]
pub async fn create_note(
    title: String,
    body: String,
    storage: State<'_, StorageState>,
) -> Result<Note, String> {
    let note = Note {
        id: uuid::Uuid::new_v4().to_string(),
        title,
        body,
        created_at: Utc::now().timestamp(),
        updated_at: Utc::now().timestamp(),
    };
    
    storage.lock().await.create_note(note).await
}

#[tauri::command]
pub async fn get_note(
    id: String,
    storage: State<'_, StorageState>,
) -> Result<NoteWithAttachments, String> {
    storage.lock().await.get_note(&id).await
}

#[tauri::command]
pub async fn list_notes(
    storage: State<'_, StorageState>,
) -> Result<Vec<Note>, String> {
    storage.lock().await.list_notes().await
}

#[tauri::command]
pub async fn update_note(
    id: String,
    title: String,
    body: String,
    storage: State<'_, StorageState>,
) -> Result<Note, String> {
    let note = Note {
        id,
        title,
        body,
        created_at: 0,
        updated_at: Utc::now().timestamp(),
    };
    
    storage.lock().await.update_note(note).await
}

#[tauri::command]
pub async fn delete_note(
    id: String,
    storage: State<'_, StorageState>,
) -> Result<(), String> {
    storage.lock().await.delete_note(&id).await
}

#[tauri::command]
pub async fn search_notes(
    query: String,
    storage: State<'_, StorageState>,
) -> Result<Vec<Note>, String> {
    storage.lock().await.search_notes(&query).await
}
