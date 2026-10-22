mod activity;
mod card;
mod engine;
mod pace;
mod scheduler;
mod settings;
mod stats;
mod updates;

use tauri::{
    menu::{IsMenuItem, Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    AppHandle, Emitter, Manager, RunEvent, WindowEvent, Wry,
};
use tauri_plugin_clipboard_manager::ClipboardExt;


/// The user pressed Start on a movement.
#[tauri::command]
fn card_started(app: AppHandle) {
    stats::record(&app, stats::Event::BreakStarted);
}

/// The user finished a card: the break timer ran out, Done on a rep movement, or Done on water.
#[tauri::command]
fn card_done(app: AppHandle, kind: String) {
    #[cfg(debug_assertions)]
    println!("[moovy] card done: {kind}");
    let event = if kind == "water" { stats::Event::WaterDone } else { stats::Event::BreakCompleted };
    stats::record(&app, event);
    log_current_card(&app, stats::Outcome::Done);
    card::hide(&app);
    engine::card_closed(&app);
}

/// The user closed the card with X.
#[tauri::command]
fn card_skip(app: AppHandle) {
    #[cfg(debug_assertions)]
    println!("[moovy] card skipped");
    stats::record(&app, stats::Event::Skipped);
    log_current_card(&app, stats::Outcome::Skipped);
    card::hide(&app);
    engine::card_closed(&app);
}

/// Puts the card on screen into today's log. Must run before the card hides.
fn log_current_card(app: &AppHandle, outcome: stats::Outcome) {
    match card::current_reminder(app) {
        Some(card::CardPayload::Break { movement_id }) => stats::log_card(app, "break", Some(movement_id), outcome),
        Some(card::CardPayload::Water) => stats::log_card(app, "water", None, outcome),
        _ => {}
    }
}

/// The Stats tab: every day in the log, newest first.
#[tauri::command]
fn get_day_logs(app: AppHandle) -> Vec<stats::DayLog> {
    stats::day_logs(&app)
}

/// The user pressed Snooze. The card has already shown its Waiting state.
/// The scheduler brings the same card back in 10 minutes.
#[tauri::command]
fn card_snooze(app: AppHandle) {
    #[cfg(debug_assertions)]
    println!("[moovy] card snoozed");
    stats::record(&app, stats::Event::Snoozed);
    card::hide(&app);
    engine::snoozed(&app);
}

/// The card's content changed size.
#[tauri::command]
fn card_resize(app: AppHandle, height: f64) {
    card::resize(&app, height);
}

/// The user grabbed the card to move it.
#[tauri::command]
fn card_drag_start(app: AppHandle) {
    card::drag_started(&app);
}

/// The hello card after onboarding was dragged into place (or closed). Nothing is counted.
#[tauri::command]
fn card_hello_done(app: AppHandle) {
    #[cfg(debug_assertions)]
    println!("[moovy] hello card done");
    card::hide(&app);
}

/// The one-time question on older installs: keep checking for updates?
#[tauri::command]
fn updates_answer(app: AppHandle, enabled: bool) {
    updates::set_enabled(&app, enabled);
    card::hide(&app);
}

/// "Install" on the update card.
#[tauri::command]
fn update_install(app: AppHandle) {
    card::hide(&app);
    updates::install(&app);
}

/// "Later" on the update card. It asks again in a day; the tray can install any time.
#[tauri::command]
fn update_later(app: AppHandle) {
    card::hide(&app);
}

fn open_settings(app: &AppHandle) {
    open_window(app, "settings", "Moovy");
}

/// "Stats" in the tray: the same window, showing the days instead of the settings.
fn open_stats(app: &AppHandle) {
    open_window(app, "stats", "Moovy · Stats");
}

fn open_window(app: &AppHandle, screen: &str, title: &str) {
    if let Some(window) = app.get_webview_window("settings") {
        let _ = app.emit_to("settings", "settings:screen", screen);
        let _ = window.set_title(title);
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

fn fast_mode_label(speed: u64) -> String {
    if speed > 1 {
        "Fast mode: on (1 min = 1 s)".into()
    } else {
        "Fast mode: off".into()
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Must be registered first: a second launch opens settings instead of a second copy.
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            open_settings(app);
        }))
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            card_started,
            card_done,
            card_skip,
            card_snooze,
            card_resize,
            card_drag_start,
            card_hello_done,
            updates_answer,
            update_install,
            update_later,
            get_day_logs,
            settings::get_settings,
            settings::set_pace,
            settings::finish_onboarding,
            settings::set_autostart,
            settings::set_updates,
            settings::set_character,
            settings::clear_data
        ])
        .setup(|app| {
            // macOS: a tray-only app, with no Dock icon and no menu bar of its own.
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let handle = app.handle();
            card::init(handle);
            engine::init(handle, settings::pace(handle));
            settings::first_run(handle);
            updates::init(handle, settings::onboarded(handle));

            let item = |id: &str, text: &str, enabled: bool| MenuItem::with_id(app, id, text, enabled, None::<&str>);
            let status = item("status", "Starting…", false)?;
            let settings_item = item("settings", "Settings", true)?;
            let stats_item = item("stats", "Stats", true)?;
            let copy_stats = item("copy_stats", "Copy my stats", true)?;
            let set_position = item("set_position", "Set card position", true)?;
            let check_updates = item("check_updates", updates::CHECK_LABEL, true)?;
            let install_update = item("install_update", "Install update", true)?;
            let quit = item("quit", "Quit", true)?;
            let fast_mode = item("fast_mode", &fast_mode_label(engine::speed(handle)), true)?;
            let fast_mode_for_menu = fast_mode.clone();

            let mut entries: Vec<Box<dyn IsMenuItem<Wry>>> = vec![
                Box::new(status.clone()),
                Box::new(PredefinedMenuItem::separator(app)?),
                Box::new(settings_item),
                Box::new(stats_item),
                Box::new(copy_stats),
                Box::new(set_position),
                Box::new(check_updates.clone()),
            ];
            // "Install update" appears right after "Check for updates" once one is downloaded.
            let install_at = entries.len();
            // Development only: run the clock 60 times faster.
            if cfg!(debug_assertions) {
                entries.push(Box::new(PredefinedMenuItem::separator(app)?));
                entries.push(Box::new(fast_mode));
            }
            entries.push(Box::new(PredefinedMenuItem::separator(app)?));
            entries.push(Box::new(quit));
            let refs: Vec<&dyn IsMenuItem<Wry>> = entries.iter().map(|e| e.as_ref()).collect();
            let menu = Menu::with_items(app, &refs)?;

            app.manage(engine::TrayItems { status });
            app.manage(updates::TrayItems { menu: menu.clone(), check: check_updates, install: install_update, install_at });

            TrayIconBuilder::with_id("moovy")
                .icon(app.default_window_icon().cloned().expect("app icon missing"))
                .tooltip("Moovy")
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "settings" => open_settings(app),
                    "stats" => open_stats(app),
                    "copy_stats" => {
                        let _ = app.clipboard().write_text(stats::summary(app));
                    }
                    "set_position" => card::set_position(app),
                    "check_updates" => updates::check(app, true),
                    "install_update" => updates::install(app),
                    "fast_mode" => {
                        let speed = if engine::speed(app) > 1 { 1 } else { engine::FAST_SPEED };
                        engine::set_speed(app, speed);
                        let _ = fast_mode_for_menu.set_text(fast_mode_label(speed));
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .build(app)?;

            #[cfg(debug_assertions)]
            {
                // Dev helper: MOOVY_DEMO_CARD=break|water shows a card 3 s after launch.
                if let Ok(kind) = std::env::var("MOOVY_DEMO_CARD") {
                    let handle = app.handle().clone();
                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_secs(3));
                        use crate::scheduler::Card;
                        engine::force_show(&handle, if kind == "water" { Card::Water } else { Card::Break });
                    });
                }
            }

            Ok(())
        })
        .on_window_event(|window, event| match event {
            // Closing the settings window only hides it. Moovy keeps running in the tray.
            WindowEvent::CloseRequested { api, .. } if window.label() == "settings" => {
                api.prevent_close();
                let _ = window.hide();
            }
            WindowEvent::Moved(_) if window.label() == "card" => {
                card::on_moved(window.app_handle());
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while building Moovy")
        .run(|_app, event| {
            // Stay alive in the tray when no window is visible. Only "Quit" exits.
            if let RunEvent::ExitRequested { api, code: None, .. } = event {
                api.prevent_exit();
            }
        });
}
