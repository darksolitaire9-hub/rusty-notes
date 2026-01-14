// src-tauri/src/commands/settings_commands.rs

use crate::settings::{Settings, save};
use tauri::{State, AppHandle, Manager};  // ✅ Add Manager here
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

type SettingsState = Arc<Mutex<Settings>>;

fn settings_file_path(app_handle: &AppHandle) -> PathBuf {
    let dir = app_handle.path().app_config_dir()
        .expect("failed to get app config dir");
    Settings::config_path(dir)
}

#[tauri::command]
pub async fn get_settings(state: State<'_, SettingsState>) -> Result<Settings, String> {
    let settings = state.lock().await;
    Ok(settings.clone())
}

#[tauri::command]
pub async fn complete_onboarding(
    state: State<'_, SettingsState>,
    app: AppHandle
) -> Result<(), String> {
    let mut settings = state.lock().await;
    if !settings.onboarding_completed {
        settings.onboarding_completed = true;
        let path = settings_file_path(&app);
        save(&path, &settings).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn update_settings(
    state: State<'_, SettingsState>,
    app: AppHandle,
    new_settings: Settings,
) -> Result<(), String> {
    let mut settings = state.lock().await;
    *settings = new_settings;
    let path = settings_file_path(&app);
    save(&path, &settings).map_err(|e| e.to_string())?;
    
    println!("✓ Settings saved: {}", settings.notes_folder);
    
    Ok(())
}
