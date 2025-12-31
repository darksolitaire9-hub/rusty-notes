// Import serialization traits to convert between Rust and JSON
use serde::{Deserialize, Serialize};

// Define the Note data structure
// Debug: allows printing for debugging
// Clone: allows making copies of notes
// Serialize/Deserialize: allows converting to/from JSON for Tauri commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,      // Unique identifier (UUID)
    pub title: String,   // Note title
    pub body: String,    // Note content (markdown)
}

// Methods for the Note struct
impl Note {
    /// Convert a note to markdown format
    /// Example: title="My Note", body="Content" 
    /// becomes: "# My Note\n\nContent"
    pub fn to_markdown(&self) -> String {
        format!("# {}\n\n{}", self.title, self.body)
    }
    
    /// Parse markdown content back into a Note
    /// Takes: note ID and the markdown file content
    /// Returns: a new Note with parsed title and body
    pub fn from_markdown(id: String, content: &str) -> Self {
        // Split the markdown into lines
        let lines: Vec<&str> = content.lines().collect();
        
        // Extract title from first line
        // "# My Note" becomes "My Note"
        let title = lines.get(0)
            .unwrap_or(&"")               // Default to empty if no first line
            .trim_start_matches("# ")     // Remove "# " prefix
            .to_string();
        
        // Extract body: skip line 0 (title) and line 1 (blank line)
        // Join remaining lines back together with newlines
        let body = lines.iter()
            .skip(2)                      // Skip first 2 lines
            .map(|s| *s)                  // Convert &str references
            .collect::<Vec<_>>()          // Collect into vector
            .join("\n");                  // Join with newlines
        
        // Create and return the Note
        Note { id, title, body }
    }
}
