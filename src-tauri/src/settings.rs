//! The few settings Moovy has: which character, the pace, start with Windows, and whether the
//! first-run onboarding is done. Stored in the same local file as the card position.

use serde::Serialize;
use serde_json::json;
use tauri::{AppHandle, Manager};
use tauri_plugin_autostart::ManagerExt as _;
use tauri_plugin_store::StoreExt;

use crate::card;
use crate::scheduler::Pace;

const STORE_FILE: &str = "moovy.json";
/// Before the pace existed (v0.1.0/0.1.1), water could only be switched on or off.
const KEY_WATER: &str = "waterEnabled";
const KEY_PACE: &str = "pace";
const KEY_CHARACTER: &str = "character";
const KEY_ONBOARDED: &str = "onboarded";
pub const DEFAULT_CHARACTER: &str = "thomas";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsView {
    pace: String,
    onboarded: bool,
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

pub fn pace(app: &AppHandle) -> Pace {
    let Ok(store) = app.store(STORE_FILE) else { return Pace::Recommended };
    if let Some(id) = store.get(KEY_PACE).and_then(|v| v.as_str().map(str::to_string)) {
        return Pace::from_id(&id);
    }
    // Older installs: water switched off meant breaks only.
    match store.get(KEY_WATER).and_then(|v| v.as_bool()) {
        Some(false) => Pace::MovesOnly,
        _ => Pace::Recommended,
    }
}

fn onboarded(app: &AppHandle) -> bool {
    app.store(STORE_FILE)
        .ok()
        .and_then(|s| s.get(KEY_ONBOARDED))
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}

/// Until onboarding is finished: start with Windows (on without asking, decided 2026-10-19)
/// and open the window, which shows the onboarding instead of the settings.
pub fn first_run(app: &AppHandle) {
    if onboarded(app) {
        return;
    }
    // Development builds never register themselves to start with Windows: that would point
    // the startup entry at a temporary debug executable.
    if !cfg!(debug_assertions) {
        let _ = app.autolaunch().enable();
    }
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

#[tauri::command]
pub fn get_settings(app: AppHandle) -> SettingsView {
    SettingsView {
        pace: pace(&app).id().to_string(),
        onboarded: onboarded(&app),
        autostart: app.autolaunch().is_enabled().unwrap_or(false),
        character: character(&app),
        version: app.package_info().version.to_string(),
    }
}

#[tauri::command]
pub fn set_pace(app: AppHandle, id: String) {
    let pace = Pace::from_id(&id);
    if let Ok(store) = app.store(STORE_FILE) {
        store.set(KEY_PACE, json!(pace.id()));
        let _ = store.save();
    }
    crate::engine::set_pace(&app, pace);
}

/// The last onboarding screen was closed: save the choices, close the window, and show the
/// hello card that explains dragging.
#[tauri::command]
pub fn finish_onboarding(app: AppHandle, character: String, pace: String) {
    let pace = Pace::from_id(&pace);
    #[cfg(debug_assertions)]
    println!("[moovy] onboarding finished: character={character} pace={}", pace.id());
    if let Ok(store) = app.store(STORE_FILE) {
        store.set(KEY_CHARACTER, json!(character));
        store.set(KEY_PACE, json!(pace.id()));
        store.set(KEY_ONBOARDED, json!(true));
        let _ = store.save();
    }
    crate::engine::set_pace(&app, pace);
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.hide();
    }
    card::show_hello(&app);
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
/// Settings (character, pace, start with Windows) are kept.
#[tauri::command]
pub fn clear_data(app: AppHandle) {
    card::clear_data(&app);
}
