// Declare the storage module
pub mod storage;

// Re-export StorageService for easy importing
// This allows: use crate::services::StorageService;
// Instead of: use crate::services::storage::StorageService;
pub use storage::StorageService;
