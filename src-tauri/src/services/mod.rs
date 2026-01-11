// Existing modules (keep unchanged)
pub mod db_storage;
pub mod storage;
pub mod trash_manager;
pub mod permanent_delete;
pub mod delete_service;
pub mod note_service;      
pub mod note_files; 

// New specialized storage modules
pub mod db_notes;
pub mod db_attachments;

// Existing public API (unchanged)
pub use db_storage::DbStorage;
pub use trash_manager::TrashManager;
pub use permanent_delete::PermanentDelete;
pub use delete_service::DeleteService;

// NEW: Export specialized storage (no #[cfg(test)])
pub use db_notes::NoteStorage;
pub use db_attachments::AttachmentStorage;



pub use note_service::NoteService;
