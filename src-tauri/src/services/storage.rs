// Import file system operations
use std::fs;
// Import path handling
use std::path::PathBuf;
// Import our Note model
use crate::models::Note;

/// Service that handles saving/loading notes to disk
pub struct StorageService {
    notes_dir: PathBuf,  // Path to the notes directory
}

impl StorageService {
    /// Create a new storage service
    /// Takes the app data directory and creates a "notes" folder inside it
    pub fn new(app_data_dir: PathBuf) -> Result<Self, String> {
        // Build path: app_data_dir/notes/
        let notes_dir = app_data_dir.join("notes");
        
        // Create the directory if it doesn't exist
        fs::create_dir_all(&notes_dir)
            .map_err(|e| format!("Failed to create notes directory: {}", e))?;
        
        Ok(StorageService { notes_dir })
    }
    
    /// Get the directory path for a specific note
    /// Example: notes/abc-123/
    fn note_dir(&self, id: &str) -> PathBuf {
        self.notes_dir.join(id)
    }
    
    /// Get the markdown file path for a note
    /// Example: notes/abc-123/note.md
    fn note_file_path(&self, id: &str) -> PathBuf {
        self.note_dir(id).join("note.md")
    }
    
    /// Save a note to disk
    pub fn save(&self, note: &Note) -> Result<(), String> {
        // Create the note's folder: notes/abc-123/
        let note_dir = self.note_dir(&note.id);
        fs::create_dir_all(&note_dir)
            .map_err(|e| format!("Failed to create note directory: {}", e))?;
        
        // Convert note to markdown format
        let content = note.to_markdown();
        
        // Build file path: notes/abc-123/note.md
        let file_path = self.note_file_path(&note.id);
        
        // Write markdown to file
        fs::write(file_path, content)
            .map_err(|e| format!("Failed to save note: {}", e))
    }
    
    /// Load a note from disk
    pub fn load(&self, id: &str) -> Result<Note, String> {
        // Build file path: notes/abc-123/note.md
        let file_path = self.note_file_path(id);
        
        // Read the entire file as text
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read note: {}", e))?;
        
        // Parse markdown back into Note struct
        Ok(Note::from_markdown(id.to_string(), &content))
    }
    
    /// List all note IDs
    pub fn list(&self) -> Result<Vec<String>, String> {
        // Read all entries in the notes directory
        let entries = fs::read_dir(&self.notes_dir)
            .map_err(|e| format!("Failed to read notes directory: {}", e))?;
        
        let mut note_ids = Vec::new();
        
        // Loop through each entry
        for entry in entries {
            if let Ok(entry) = entry {
                // Check if it's a directory (not a file)
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    if let Some(dirname) = entry.file_name().to_str() {
                        // Check if note.md exists inside the folder
                        let note_path = entry.path().join("note.md");
                        if note_path.exists() {
                            // This is a valid note folder
                            note_ids.push(dirname.to_string());
                        }
                    }
                }
            }
        }
        
        Ok(note_ids)
    }
    
    /// Delete a note and its folder
    pub fn delete(&self, id: &str) -> Result<(), String> {
        // Get the note's directory
        let note_dir = self.note_dir(id);
        
        // Remove the entire folder (including note.md and future attachments)
        fs::remove_dir_all(note_dir)
            .map_err(|e| format!("Failed to delete note: {}", e))
    }
}
