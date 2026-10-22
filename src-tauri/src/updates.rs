//! Updates (decided 2026-10-22): on by default, shown openly in onboarding and settings.
//!
//! - Once a day Moovy asks GitHub for `latest.json`, the version number of the newest release.
//!   Nothing about the user is sent. Switched off, Moovy makes no network request at all.
//! - A newer version downloads quietly and its signature is checked against the key built into
//!   the app. Only then does the character ask: install now, or later.
//! - Nothing installs without the user saying so, and the card waits for a quiet moment.

use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde_json::json;
use tauri::{
    menu::{Menu, MenuItem},
    AppHandle, Manager, Wry,
};
use tauri_plugin_store::StoreExt;
use tauri_plugin_updater::{Update, UpdaterExt};

use crate::card::{self, CardPayload};
use crate::engine;

const STORE_FILE: &str = "moovy.json";
const KEY_UPDATES: &str = "updates";
const KEY_LAST_CHECK: &str = "lastUpdateCheck";

const CHECK_EVERY: Duration = Duration::from_secs(24 * 60 * 60);
/// The first look after launch waits a little, so Moovy starts without touching the network.
const FIRST_LOOK: Duration = Duration::from_secs(2 * 60);
const LOOK_EVERY: Duration = Duration::from_secs(30);

#[derive(Default)]
struct UpdateRuntime {
    /// Downloaded and signature-checked, waiting for the user.
    ready: Option<(Update, Vec<u8>)>,
    checking: bool,
    /// The "install now?" card is due at the next quiet moment.
    ask_due: bool,
    /// The one-time "keep update checks on?" card is due (installs from before 0.1.7).
    notice_due: bool,
    /// When the ask card was last shown, so "Later" means a day later.
    asked_at: Option<SystemTime>,
}

type UpdateState = Mutex<UpdateRuntime>;

/// The tray lines for updates. "Install update" joins the menu only once there is one.
pub struct TrayItems {
    pub menu: Menu<Wry>,
    pub check: MenuItem<Wry>,
    pub install: MenuItem<Wry>,
    /// Where "Install update" goes in the menu.
    pub install_at: usize,
}

pub const CHECK_LABEL: &str = "Check for updates";

/// The saved choice. `None` on installs from before update checks existed.
fn saved(app: &AppHandle) -> Option<bool> {
    app.store(STORE_FILE).ok()?.get(KEY_UPDATES)?.as_bool()
}

pub fn enabled(app: &AppHandle) -> bool {
    saved(app).unwrap_or(true)
}

pub fn set_enabled(app: &AppHandle, enabled: bool) {
    if let Ok(store) = app.store(STORE_FILE) {
        store.set(KEY_UPDATES, json!(enabled));
        let _ = store.save();
    }
    app.state::<UpdateState>().lock().unwrap().notice_due = false;
}

fn now_secs() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

/// Is a daily check due? `last` is when the last one ran, in Unix seconds.
fn check_due(enabled: bool, last: Option<u64>, now: u64) -> bool {
    enabled && last.is_none_or(|last| now.saturating_sub(last) >= CHECK_EVERY.as_secs() || last > now)
}

/// Starts the daily routine. `onboarded` tells an existing install (which gets the one-time
/// notice) from a new one (which chose in onboarding).
pub fn init(app: &AppHandle, onboarded: bool) {
    app.manage(Mutex::new(UpdateRuntime {
        notice_due: onboarded && saved(app).is_none(),
        ..Default::default()
    }));
    let app = app.clone();
    std::thread::spawn(move || {
        std::thread::sleep(FIRST_LOOK);
        loop {
            look(&app);
            std::thread::sleep(LOOK_EVERY);
        }
    });
}

fn look(app: &AppHandle) {
    let last = app
        .store(STORE_FILE)
        .ok()
        .and_then(|s| s.get(KEY_LAST_CHECK))
        .and_then(|v| v.as_u64());
    // Until the one-time notice is answered, the old promise (no network) still holds.
    let notice_due = app.state::<UpdateState>().lock().unwrap().notice_due;
    if !notice_due && check_due(enabled(app), last, now_secs()) {
        check(app, false);
    }
    maybe_show_card(app);
}

/// Asks GitHub for the newest version and, if it is newer, downloads it in the background.
/// `by_hand`: "Check for updates" in the tray, which works even with the switch off, asks
/// right away and says when there is nothing new.
pub fn check(app: &AppHandle, by_hand: bool) {
    {
        let state = app.state::<UpdateState>();
        let mut u = state.lock().unwrap();
        if u.checking {
            return;
        }
        if u.ready.is_some() {
            // Already downloaded: a new day, or asking by hand, brings the question back.
            if by_hand || u.asked_at.is_some_and(|at| at.elapsed().unwrap_or_default() >= CHECK_EVERY) {
                u.ask_due = true;
            }
            return;
        }
        u.checking = true;
    }
    if by_hand {
        tray_check_text(app, "Checking for updates…");
    }
    if let Ok(store) = app.store(STORE_FILE) {
        store.set(KEY_LAST_CHECK, json!(now_secs()));
        let _ = store.save();
    }
    let app = app.clone();
    tauri::async_runtime::spawn(async move {
        let result = fetch(&app).await;
        let state = app.state::<UpdateState>();
        let mut u = state.lock().unwrap();
        u.checking = false;
        match result {
            Ok(Some(ready)) => {
                #[cfg(debug_assertions)]
                println!("[moovy] update {} downloaded and verified", ready.0.version);
                let version = ready.0.version.clone();
                u.ready = Some(ready);
                u.ask_due = true;
                drop(u);
                show_tray_item(&app, &version);
                if by_hand {
                    tray_check_text(&app, CHECK_LABEL);
                    maybe_show_card(&app);
                }
            }
            Ok(None) => {
                drop(u);
                #[cfg(debug_assertions)]
                println!("[moovy] no update");
                if by_hand {
                    let version = app.package_info().version.to_string();
                    tray_check_text_for_a_while(&app, format!("Up to date (v{version})"));
                }
            }
            Err(_e) => {
                drop(u);
                #[cfg(debug_assertions)]
                println!("[moovy] update check failed: {_e}");
                if by_hand {
                    tray_check_text_for_a_while(&app, "Couldn't check. Try again later".into());
                }
            }
        }
    });
}

async fn fetch(app: &AppHandle) -> Result<Option<(Update, Vec<u8>)>, String> {
    let updater = updater(app)?;
    let Some(update) = updater.check().await.map_err(|e| e.to_string())? else {
        return Ok(None);
    };
    // `download` checks the signature and fails if it doesn't match the built-in key.
    let bytes = update.download(|_, _| {}, || {}).await.map_err(|e| e.to_string())?;
    Ok(Some((update, bytes)))
}

fn updater(app: &AppHandle) -> Result<tauri_plugin_updater::Updater, String> {
    let builder = app.updater_builder();
    // Testing only: a test build can point at a local file server. The address must be on this
    // computer, and the signature is checked all the same.
    let builder = match std::env::var("MOOVY_UPDATE_URL").ok().filter(|u| u.starts_with("http://127.0.0.1:")) {
        Some(url) => builder
            .endpoints(vec![tauri::Url::parse(&url).map_err(|e| e.to_string())?])
            .map_err(|e| e.to_string())?,
        None => builder,
    };
    builder.build().map_err(|e| e.to_string())
}

/// Shows a waiting update card if nothing else is on screen and the user isn't in a call,
/// presenting or away.
fn maybe_show_card(app: &AppHandle) {
    if card::is_showing(app) || !engine::is_quiet(app) {
        return;
    }
    let payload = {
        let state = app.state::<UpdateState>();
        let mut u = state.lock().unwrap();
        if u.notice_due {
            Some(CardPayload::UpdatesNotice)
        } else if u.ask_due && let Some((update, _)) = &u.ready {
            let version = update.version.clone();
            u.ask_due = false;
            u.asked_at = Some(SystemTime::now());
            Some(CardPayload::UpdateReady { version })
        } else {
            None
        }
    };
    if let Some(payload) = payload {
        let handle = app.clone();
        let _ = app.run_on_main_thread(move || card::show_notice(&handle, payload));
    }
}

fn show_tray_item(app: &AppHandle, version: &str) {
    let Some(tray) = app.try_state::<TrayItems>() else { return };
    let _ = tray.install.set_text(format!("Install update (v{version})"));
    let already = tray.menu.get(tray.install.id()).is_some();
    if !already {
        let _ = tray.menu.insert(&tray.install, tray.install_at);
    }
}

fn tray_check_text(app: &AppHandle, text: &str) {
    if let Some(tray) = app.try_state::<TrayItems>() {
        let _ = tray.check.set_text(text);
    }
}

/// A short answer in place of "Check for updates", then back to normal.
fn tray_check_text_for_a_while(app: &AppHandle, text: String) {
    tray_check_text(app, &text);
    let app = app.clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(20));
        tray_check_text(&app, CHECK_LABEL);
    });
}

/// Installs the downloaded update. Windows runs the installer, which closes Moovy and opens the
/// new version. On macOS the app is replaced in place and Moovy starts again.
pub fn install(app: &AppHandle) {
    let Some((update, bytes)) = app.state::<UpdateState>().lock().unwrap().ready.take() else {
        return;
    };
    match update.install(bytes) {
        Ok(()) => {
            #[cfg(not(windows))]
            {
                // The new copy must not find this one still holding the single-instance lock.
                tauri_plugin_single_instance::destroy(app);
                app.restart();
            }
        }
        Err(_e) => {
            #[cfg(debug_assertions)]
            println!("[moovy] update install failed: {_e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: u64 = 24 * 60 * 60;

    #[test]
    fn off_never_checks() {
        assert!(!check_due(false, None, 10 * DAY));
        assert!(!check_due(false, Some(0), 10 * DAY));
    }

    #[test]
    fn checks_once_a_day() {
        assert!(check_due(true, None, 5));
        assert!(!check_due(true, Some(DAY), DAY + 60));
        assert!(!check_due(true, Some(DAY), 2 * DAY - 1));
        assert!(check_due(true, Some(DAY), 2 * DAY));
    }

    #[test]
    fn a_clock_set_back_does_not_block_checks() {
        assert!(check_due(true, Some(5 * DAY), DAY));
    }
}
