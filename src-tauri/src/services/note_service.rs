// src-tauri/src/services/note_service.rs
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::Utc;
use uuid::Uuid;

use crate::services::db_storage::DbStorage;
use crate::db::schema::{Note, NoteWithAttachments};
use crate::settings::Settings;
use crate::services::note_files::NoteFileStore;

/// High-level note operations (coordinates DB + files)
pub struct NoteService {
    storage: Arc<Mutex<DbStorage>>,
    settings: Arc<Mutex<Settings>>,
}

impl NoteService {
    pub fn new(storage: Arc<Mutex<DbStorage>>, settings: Arc<Mutex<Settings>>) -> Self {
        Self { storage, settings }
    }

    /// Creates a new note (DB + file)
    pub async fn create(&self, title: String, body: String) -> Result<Note, String> {
        // Get notes folder
        let settings = self.settings.lock().await;
        let notes_folder = settings.notes_folder.clone();
        drop(settings);

        // Setup file store
        let file_store = NoteFileStore::new(notes_folder);
        file_store.ensure_folder_exists().await?;

        // Generate note metadata
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().timestamp();
        let file_path = file_store.path_for_id(&id);

        // Build note
        let note = Note {
            id: id.clone(),
            title,
            body: body.clone(),
            created_at: now,
            updated_at: now,
            file_path: file_path.clone(),
        };

        // 1) Save to DB
        let created_note = self.storage.lock().await.create_note(note).await?;

        // 2) Write file
        file_store.write_note_file(&file_path, &body).await?;

        println!("✅ Created note: {} at {}", created_note.id, file_path);
        Ok(created_note)
    }

    /// Updates existing note (DB + file)
    pub async fn update(&self, id: String, title: String, body: String) -> Result<Note, String> {
        // Load existing note
        let mut existing = self.storage.lock().await.get_note(&id).await?.note;

        // Update fields
        existing.title = title;
        existing.body = body.clone();
        existing.updated_at = Utc::now().timestamp();

        // 1) Update DB
        let updated_note = self.storage.lock().await.update_note(existing).await?;

        // 2) Write file
        let settings = self.settings.lock().await;
        let file_store = NoteFileStore::new(settings.notes_folder.clone());
        drop(settings);

        file_store.write_note_file(&updated_note.file_path, &body).await?;

        println!("✅ Updated note: {} at {}", updated_note.id, updated_note.file_path);
        Ok(updated_note)
    }

    /// Gets note with attachments
    pub async fn get(&self, id: String) -> Result<NoteWithAttachments, String> {
        self.storage.lock().await.get_note(&id).await
    }

    /// Lists all notes
    pub async fn list(&self) -> Result<Vec<Note>, String> {
        self.storage.lock().await.list_notes().await
    }

    /// Searches notes
    pub async fn search(&self, query: String) -> Result<Vec<Note>, String> {
        self.storage.lock().await.search_notes(&query).await
    }
}
