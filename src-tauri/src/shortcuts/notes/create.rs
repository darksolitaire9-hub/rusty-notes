use tauri::{AppHandle, Emitter};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

pub fn register(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let shortcut: Shortcut = "Ctrl+N".parse()?;

    app.global_shortcut()
        .on_shortcut(shortcut, move |app, _shortcut, event| {
            // Correct order: app, shortcut, event
            if event.state() == ShortcutState::Pressed {
                if let Err(e) = app.emit("create-new-note", ()) {
                    eprintln!("Failed to emit create-new-note event: {e}");
                }
            }
        })?;

    println!("Registered shortcut: Ctrl+N");
    Ok(())
}

pub fn unregister(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let shortcut: Shortcut = "Ctrl+N".parse()?;
    app.global_shortcut().unregister(shortcut)?;
    Ok(())
}
