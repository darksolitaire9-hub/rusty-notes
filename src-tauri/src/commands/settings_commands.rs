use crate::settings::{Settings, save};
use crate::state::SettingsState;
use tauri::{State, AppHandle, Manager};
use std::path::PathBuf;


fn settings_file_path(app_handle: &AppHandle) -> PathBuf {
    let dir = app_handle.path().app_config_dir()
        .expect("failed to get app config dir");
    Settings::config_path(dir)
}

#[tauri::command]
pub fn get_settings(state: State<SettingsState>) -> Settings {
    let settings = state.inner.lock().expect("mutex poisoned");
    settings.clone()
}

#[tauri::command]
pub fn complete_onboarding(state: State<SettingsState>, app: AppHandle) -> Result<(), String> {
    let mut settings = state.inner.lock().map_err(|_| "mutex poisoned")?;
    if !settings.onboarding_completed {
        settings.onboarding_completed = true;
        let path = settings_file_path(&app);
        save(&path, &settings).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn update_settings(
    state: State<SettingsState>,
    app: AppHandle,
    new_settings: Settings,
) -> Result<(), String> {
    let mut settings = state.inner.lock().map_err(|_| "mutex poisoned")?;
    *settings = new_settings;
    let path = settings_file_path(&app);
    save(&path, &settings).map_err(|e| e.to_string())?;
    Ok(())
}
