use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub version: u32,
    pub notes_folder: String,
    pub auto_save_interval_secs: u64,
    pub delete_behavior: DeleteBehavior,
    pub onboarding_completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeleteBehavior {
    MoveToTrash,
    Permanent,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            version: 1,
            notes_folder: String::from("./notes"),
            auto_save_interval_secs: 30,
            delete_behavior: DeleteBehavior::MoveToTrash,
            onboarding_completed: false,
        }
    }
}

impl Settings {
    pub fn config_path(app_config_dir: std::path::PathBuf) -> std::path::PathBuf {
        app_config_dir.join("settings.toml")
    }
}
