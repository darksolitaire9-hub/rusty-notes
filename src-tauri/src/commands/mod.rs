// Declare the notes commands module
pub mod notes;

// Re-export all command functions
// This makes them available at the crate::commands level
pub use notes::*;
