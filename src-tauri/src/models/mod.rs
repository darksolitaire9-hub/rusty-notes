// Export the note module so other files can use it
pub mod note;

// Re-export Note struct for convenience
// This allows: use crate::models::Note;
// Instead of: use crate::models::note::Note;
pub use note::Note;
