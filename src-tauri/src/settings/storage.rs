use crate::settings::model::Settings;
use std::{fs, path::PathBuf};

pub fn load_or_init(app_config_dir: PathBuf) -> anyhow::Result<Settings> {
    let path = Settings::config_path(app_config_dir);

    if !path.exists() {
        let settings = Settings::default();
        save(&path, &settings)?;
        return Ok(settings);
    }

    let contents = fs::read_to_string(&path)?;
    let mut loaded: Settings = toml::from_str(&contents)?;

    // simple migration: ensure version and future defaults
    if loaded.version == 0 {
        loaded.version = 1;
    }

    save(&path, &loaded)?; // normalize file
    Ok(loaded)
}

pub fn save(path: &PathBuf, settings: &Settings) -> anyhow::Result<()> {
    let toml_str = toml::to_string_pretty(settings)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, toml_str)?;
    Ok(())
}
