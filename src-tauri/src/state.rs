use crate::settings::Settings;
use std::sync::Mutex;

pub struct SettingsState {
    pub inner: Mutex<Settings>,
}
