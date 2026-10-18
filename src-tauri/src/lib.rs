mod card;
mod settings;
mod stats;

use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager, RunEvent, WindowEvent,
};
use tauri_plugin_clipboard_manager::ClipboardExt;

/// The user pressed Start on a movement.
#[tauri::command]
fn card_started(app: AppHandle) {
    stats::record(&app, stats::Event::BreakStarted);
}

/// The user finished a card: the break timer ran out, or they pressed Done on water.
#[tauri::command]
fn card_done(app: AppHandle, kind: String) {
    #[cfg(debug_assertions)]
    println!("[moovy] card done: {kind}");
    let event = if kind == "water" { stats::Event::WaterDone } else { stats::Event::BreakCompleted };
    stats::record(&app, event);
    card::hide(&app);
}

/// The user closed the card with X.
#[tauri::command]
fn card_skip(app: AppHandle) {
    #[cfg(debug_assertions)]
    println!("[moovy] card skipped");
    stats::record(&app, stats::Event::Skipped);
    card::hide(&app);
}

/// The user pressed Snooze. The card has already shown its Waiting state.
#[tauri::command]
fn card_snooze(app: AppHandle) {
    #[cfg(debug_assertions)]
    println!("[moovy] card snoozed for {:?}", card::snooze_duration());
    stats::record(&app, stats::Event::Snoozed);
    card::snooze(&app);
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

fn open_settings(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
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
        .invoke_handler(tauri::generate_handler![
            card_started,
            card_done,
            card_skip,
            card_snooze,
            card_resize,
            card_drag_start,
            settings::get_settings,
            settings::set_water_enabled,
            settings::set_autostart,
            settings::set_character,
            settings::clear_data
        ])
        .setup(|app| {
            card::init(app.handle());
            settings::first_run(app.handle());

            let show_break = MenuItem::with_id(app, "show_break", "Show card: Break (next movement)", true, None::<&str>)?;
            let show_water = MenuItem::with_id(app, "show_water", "Show card: Water", true, None::<&str>)?;
            let show_delayed = MenuItem::with_id(app, "show_delayed", "Show card in 5 seconds (focus test)", true, None::<&str>)?;
            let reset_position = MenuItem::with_id(app, "reset_position", "Reset card position", true, None::<&str>)?;
            let copy_stats = MenuItem::with_id(app, "copy_stats", "Copy my stats", true, None::<&str>)?;
            let open = MenuItem::with_id(app, "open", "Open Moovy", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let separator = PredefinedMenuItem::separator(app)?;
            let separator2 = PredefinedMenuItem::separator(app)?;

            let menu = Menu::with_items(
                app,
                &[
                    &show_break,
                    &show_water,
                    &show_delayed,
                    &separator,
                    &reset_position,
                    &copy_stats,
                    &open,
                    &separator2,
                    &quit,
                ],
            )?;

            TrayIconBuilder::with_id("moovy")
                .icon(app.default_window_icon().cloned().expect("app icon missing"))
                .tooltip("Moovy")
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show_break" => card::show_next_break(app),
                    "show_water" => card::show(app, card::CardPayload::Water),
                    "show_delayed" => {
                        // Gives time to click into another app and keep typing.
                        let handle = app.clone();
                        std::thread::spawn(move || {
                            std::thread::sleep(std::time::Duration::from_secs(5));
                            let inner = handle.clone();
                            let _ = handle.run_on_main_thread(move || card::show_next_break(&inner));
                        });
                    }
                    "reset_position" => card::reset_position(app),
                    "copy_stats" => {
                        let _ = app.clipboard().write_text(stats::summary(app));
                    }
                    "open" => open_settings(app),
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
                        let inner = handle.clone();
                        let _ = handle.run_on_main_thread(move || match kind.as_str() {
                            "water" => card::show(&inner, card::CardPayload::Water),
                            _ => card::show_next_break(&inner),
                        });
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
