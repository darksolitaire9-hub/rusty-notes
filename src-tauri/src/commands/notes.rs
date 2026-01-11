// src-tauri/src/commands/notes.rs
use tauri::State;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::services::{DbStorage, DeleteService, NoteService};
use crate::db::schema::{Note, NoteWithAttachments};
use crate::settings::Settings;

type StorageState = Arc<Mutex<DbStorage>>;
type SettingsState = Arc<Mutex<Settings>>;

#[tauri::command]
pub async fn create_note(
    title: String,
    body: String,
    storage: State<'_, StorageState>,
    settings_state: State<'_, SettingsState>,
) -> Result<Note, String> {
    let service = NoteService::new(storage.inner().clone(), settings_state.inner().clone());
    service.create(title, body).await
}

#[tauri::command]
pub async fn update_note(
    id: String,
    title: String,
    body: String,
    storage: State<'_, StorageState>,
    settings_state: State<'_, SettingsState>,
) -> Result<Note, String> {
    let service = NoteService::new(storage.inner().clone(), settings_state.inner().clone());
    service.update(id, title, body).await
}

#[tauri::command]
pub async fn get_note(
    id: String,
    storage: State<'_, StorageState>,
    settings_state: State<'_, SettingsState>,
) -> Result<NoteWithAttachments, String> {
    let service = NoteService::new(storage.inner().clone(), settings_state.inner().clone());
    service.get(id).await
}

#[tauri::command]
pub async fn list_notes(
    storage: State<'_, StorageState>,
    settings_state: State<'_, SettingsState>,
) -> Result<Vec<Note>, String> {
    let service = NoteService::new(storage.inner().clone(), settings_state.inner().clone());
    service.list().await
}

#[tauri::command]
pub async fn search_notes(
    query: String,
    storage: State<'_, StorageState>,
    settings_state: State<'_, SettingsState>,
) -> Result<Vec<Note>, String> {
    let service = NoteService::new(storage.inner().clone(), settings_state.inner().clone());
    service.search(query).await
}

#[tauri::command]
pub async fn delete_note(
    id: String,
    storage: State<'_, StorageState>,
    settings_state: State<'_, SettingsState>,
) -> Result<(), String> {
    let settings = settings_state.lock().await;
    let notes_folder = settings.notes_folder.clone();
    
    let note_with_attachments = storage.lock().await.get_note(&id).await?;
    
    DeleteService::delete_note(&notes_folder, &note_with_attachments, &settings).await?;
    storage.lock().await.delete_note(&id).await
}
