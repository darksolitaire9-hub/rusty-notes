// src-tauri/src/settings/mod.rs

pub mod model;
pub mod storage; 

pub use model::Settings;
pub use storage::{load_or_init, save};

