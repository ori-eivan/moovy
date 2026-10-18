//! The few settings Moovy has: which character, water reminders on or off, start with Windows.
//! Stored in the same local file as the card position.

use serde::Serialize;
use serde_json::json;
use tauri::{AppHandle, Manager};
use tauri_plugin_autostart::ManagerExt as _;
use tauri_plugin_store::StoreExt;

use crate::card;

const STORE_FILE: &str = "moovy.json";
const KEY_WATER: &str = "waterEnabled";
const KEY_CHARACTER: &str = "character";
const KEY_ONBOARDED: &str = "onboarded";
pub const DEFAULT_CHARACTER: &str = "pilot-1";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsView {
    water_enabled: bool,
    autostart: bool,
    character: String,
    version: String,
}

pub fn character(app: &AppHandle) -> String {
    app.store(STORE_FILE)
        .ok()
        .and_then(|s| s.get(KEY_CHARACTER))
        .and_then(|v| v.as_str().map(str::to_string))
        .unwrap_or_else(|| DEFAULT_CHARACTER.to_string())
}

pub fn water_enabled(app: &AppHandle) -> bool {
    app.store(STORE_FILE)
        .ok()
        .and_then(|s| s.get(KEY_WATER))
        .and_then(|v| v.as_bool())
        .unwrap_or(true)
}

/// First launch: turn on start-with-Windows (the agreed default, visible in the window)
/// and open the settings window so the user can pick a character.
pub fn first_run(app: &AppHandle) {
    let Ok(store) = app.store(STORE_FILE) else { return };
    if store.get(KEY_ONBOARDED).and_then(|v| v.as_bool()) == Some(true) {
        return;
    }
    // Development builds never register themselves to start with Windows: that would point
    // the startup entry at a temporary debug executable.
    if !cfg!(debug_assertions) {
        let _ = app.autolaunch().enable();
    }
    store.set(KEY_ONBOARDED, json!(true));
    let _ = store.save();
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

#[tauri::command]
pub fn get_settings(app: AppHandle) -> SettingsView {
    SettingsView {
        water_enabled: water_enabled(&app),
        autostart: app.autolaunch().is_enabled().unwrap_or(false),
        character: character(&app),
        version: app.package_info().version.to_string(),
    }
}

#[tauri::command]
pub fn set_water_enabled(app: AppHandle, enabled: bool) {
    if let Ok(store) = app.store(STORE_FILE) {
        store.set(KEY_WATER, json!(enabled));
        let _ = store.save();
    }
}

#[tauri::command]
pub fn set_autostart(app: AppHandle, enabled: bool) -> Result<(), String> {
    // A development build would register a temporary debug executable that only works while
    // the dev server runs. Turning it off is always allowed.
    if enabled && cfg!(debug_assertions) {
        return Err("Start with Windows is only available in the installed app".into());
    }
    let autolaunch = app.autolaunch();
    let result = if enabled { autolaunch.enable() } else { autolaunch.disable() };
    result.map_err(|e| e.to_string())
}

/// Picking a character also shows a card with it, so the choice is seen right away.
#[tauri::command]
pub fn set_character(app: AppHandle, id: String) {
    if let Ok(store) = app.store(STORE_FILE) {
        store.set(KEY_CHARACTER, json!(id));
        let _ = store.save();
    }
    card::preview_next_break(&app);
}

/// "Clear my data": forgets the card position, the movement rotation and the daily counts.
/// Settings (character, water, start with Windows) are kept.
#[tauri::command]
pub fn clear_data(app: AppHandle) {
    card::clear_data(&app);
}
