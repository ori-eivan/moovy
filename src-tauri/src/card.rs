//! The reminder card window.
//!
//! - Shown at the bottom-left of the work area of the monitor the user is working on, or
//!   wherever the user last dragged it (remembered automatically, no settings needed).
//! - Never takes keyboard focus.
//! - Grows and shrinks with its content, keeping its bottom edge in place.

use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_store::StoreExt;

use crate::stats;

/// Logical width of the card window. The card is 360 px wide; the rest is room for its shadow.
pub const CARD_WIDTH: f64 = 380.0;
/// Starting height. The card resizes the window to fit its content right after it renders.
pub const CARD_HEIGHT: f64 = 200.0;
/// Gap between the card and the edges of the work area, in logical px.
const MARGIN: f64 = 16.0;

const STORE_FILE: &str = "moovy.json";
const KEY_POSITION: &str = "cardPosition";
const KEY_ROTATION: &str = "rotationIndex";

/// Breaks alternate Movement and Stretch. Within each, the three rotate in order.
const ROTATION: [&str; 6] = [
    "wall-angels",
    "neck-flexion",
    "spine-twist",
    "upper-trap",
    "hip-flexor",
    "doorway-chest",
];

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CardPayload {
    Break {
        #[serde(rename = "movementId")]
        movement_id: String,
    },
    Water,
}

/// Where the user parked the card: the card's bottom-left corner, as a fraction of the work area.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct SavedPosition {
    pub fx: f64,
    pub fy: f64,
}

#[derive(Default)]
pub struct CardRuntime {
    current: Option<CardPayload>,
    /// The last card shown, kept after it closes so a snoozed card can come back.
    last: Option<CardPayload>,
    saved_position: Option<SavedPosition>,
    rotation_index: usize,
    /// True between the user grabbing the card and the card hiding. Only drags are remembered.
    dragging: bool,
}

pub type CardState = Mutex<CardRuntime>;

/// Reads the remembered position and rotation from disk. Called once at startup.
pub fn init(app: &AppHandle) {
    let mut runtime = CardRuntime::default();
    if let Ok(store) = app.store(STORE_FILE) {
        runtime.saved_position = store
            .get(KEY_POSITION)
            .and_then(|v| serde_json::from_value(v).ok());
        runtime.rotation_index = store
            .get(KEY_ROTATION)
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize;
    }
    app.manage(Mutex::new(runtime));
}

/// Shows the next movement in the rotation.
pub fn show_next_break(app: &AppHandle) {
    let payload = next_break(app);
    show(app, payload);
}

/// Same card as `show_next_break`, but not counted in the stats (used to preview a character).
pub fn preview_next_break(app: &AppHandle) {
    let payload = next_break(app);
    present(app, payload);
}

fn next_break(app: &AppHandle) -> CardPayload {
    let movement_id = {
        let state = app.state::<CardState>();
        let mut runtime = state.lock().unwrap();
        let id = ROTATION[runtime.rotation_index % ROTATION.len()].to_string();
        runtime.rotation_index = (runtime.rotation_index + 1) % ROTATION.len();
        if let Ok(store) = app.store(STORE_FILE) {
            store.set(KEY_ROTATION, json!(runtime.rotation_index));
            let _ = store.save();
        }
        id
    };
    CardPayload::Break { movement_id }
}

/// Shows a new card and counts it.
pub fn show(app: &AppHandle, payload: CardPayload) {
    stats::record(
        app,
        match payload {
            CardPayload::Break { .. } => stats::Event::BreakShown,
            CardPayload::Water => stats::Event::WaterShown,
        },
    );
    present(app, payload);
}

/// Puts a card on screen without counting it (snooze returns, previews).
fn present(app: &AppHandle, payload: CardPayload) {
    let Some(window) = app.get_webview_window("card") else {
        return;
    };
    let saved = {
        let state = app.state::<CardState>();
        let mut runtime = state.lock().unwrap();
        runtime.current = Some(payload.clone());
        runtime.last = Some(payload.clone());
        runtime.dragging = false;
        runtime.saved_position
    };
    let _ = app.emit_to("card", "card:show", &payload);

    #[cfg(windows)]
    win32::place_and_show(&window, saved);

    #[cfg(not(windows))]
    {
        let _ = saved;
        let _ = window.show();
    }
}

pub fn hide(app: &AppHandle) {
    let was_dragged = {
        let state = app.state::<CardState>();
        let mut runtime = state.lock().unwrap();
        runtime.current = None;
        std::mem::take(&mut runtime.dragging)
    };
    // If the user moved the card while it was up, remember where it ended. Move events are
    // not reliable during the drag itself, so the final rectangle is read here.
    if was_dragged {
        #[cfg(windows)]
        if let Some(position) = app
            .get_webview_window("card")
            .and_then(|window| win32::current_position(&window))
        {
            app.state::<CardState>().lock().unwrap().saved_position = Some(position);
            if let Ok(store) = app.store(STORE_FILE) {
                store.set(KEY_POSITION, json!(position));
            }
        }
    }
    if let Ok(store) = app.store(STORE_FILE) {
        let _ = store.save();
    }
    if let Some(window) = app.get_webview_window("card") {
        // Hidden the same way it was shown (directly through Windows), otherwise Tauri
        // believes the window is already hidden and skips the call.
        #[cfg(windows)]
        win32::hide(&window);
        #[cfg(not(windows))]
        let _ = window.hide();
    }
}

/// Brings back the last card (after a snooze). The scheduler decides when.
/// The same card coming back is not a new card, so it is not counted again.
pub fn present_last(app: &AppHandle) {
    let last = app.state::<CardState>().lock().unwrap().last.clone();
    if let Some(payload) = last {
        present(app, payload);
    }
}

pub fn drag_started(app: &AppHandle) {
    let state = app.state::<CardState>();
    state.lock().unwrap().dragging = true;
}

/// Called for every move of the card window. Only moves made by the user dragging are remembered.
pub fn on_moved(app: &AppHandle) {
    let dragging = app.state::<CardState>().lock().unwrap().dragging;
    if !dragging {
        return;
    }
    let Some(window) = app.get_webview_window("card") else {
        return;
    };
    #[cfg(windows)]
    if let Some(position) = win32::current_position(&window) {
        let state = app.state::<CardState>();
        state.lock().unwrap().saved_position = Some(position);
        if let Ok(store) = app.store(STORE_FILE) {
            store.set(KEY_POSITION, json!(position));
        }
    }
    #[cfg(not(windows))]
    let _ = window;
}

/// Forgets everything the card remembered: its position and where the rotation is.
pub fn clear_data(app: &AppHandle) {
    {
        let state = app.state::<CardState>();
        let mut runtime = state.lock().unwrap();
        runtime.saved_position = None;
        runtime.rotation_index = 0;
    }
    if let Ok(store) = app.store(STORE_FILE) {
        store.delete(KEY_POSITION);
        store.delete(KEY_ROTATION);
        let _ = store.save();
    }
    stats::clear(app);
}

pub fn reset_position(app: &AppHandle) {
    let state = app.state::<CardState>();
    state.lock().unwrap().saved_position = None;
    if let Ok(store) = app.store(STORE_FILE) {
        store.delete(KEY_POSITION);
        let _ = store.save();
    }
}

/// Resizes the card window to `logical_height`, keeping its bottom edge where it is.
pub fn resize(app: &AppHandle, logical_height: f64) {
    if let Some(window) = app.get_webview_window("card") {
        #[cfg(windows)]
        win32::resize_keep_bottom(&window, logical_height);
        #[cfg(not(windows))]
        let _ = (window, logical_height);
    }
}

/// A rectangle in physical pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    fn width(&self) -> i32 {
        self.right - self.left
    }
    fn height(&self) -> i32 {
        self.bottom - self.top
    }
}

/// Where the card window's top-left corner goes, in physical pixels.
/// Without a saved position: bottom-left of the work area, `MARGIN` from both edges.
/// With one: the saved spot scaled to this work area, kept fully inside it.
pub fn placement(work: Rect, scale: f64, width: i32, height: i32, saved: Option<SavedPosition>) -> (i32, i32) {
    let (x, bottom) = match saved {
        None => {
            let margin = (MARGIN * scale).round() as i32;
            (work.left + margin, work.bottom - margin)
        }
        Some(p) => (
            work.left + (p.fx * work.width() as f64).round() as i32,
            work.top + (p.fy * work.height() as f64).round() as i32,
        ),
    };
    let x = x.clamp(work.left, (work.right - width).max(work.left));
    let y = (bottom - height).clamp(work.top, (work.bottom - height).max(work.top));
    (x, y)
}

/// The inverse of `placement`: turns a window rectangle back into a saved fraction.
pub fn to_saved(work: Rect, window: Rect) -> SavedPosition {
    let fx = (window.left - work.left) as f64 / work.width().max(1) as f64;
    let fy = (window.bottom - work.top) as f64 / work.height().max(1) as f64;
    SavedPosition {
        fx: fx.clamp(0.0, 1.0),
        fy: fy.clamp(0.0, 1.0),
    }
}

#[cfg(windows)]
mod win32 {
    use super::{placement, to_saved, Rect, SavedPosition, CARD_HEIGHT, CARD_WIDTH};
    use windows::Win32::Foundation::{HWND, RECT};
    use windows::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MonitorFromWindow, HMONITOR, MONITORINFO, MONITOR_DEFAULTTONEAREST,
        MONITOR_DEFAULTTOPRIMARY,
    };
    use windows::Win32::UI::HiDpi::{GetDpiForMonitor, GetDpiForWindow, MDT_EFFECTIVE_DPI};
    use windows::Win32::UI::WindowsAndMessaging::{
        GetForegroundWindow, GetWindowLongPtrW, GetWindowRect, SetWindowDisplayAffinity,
        SetWindowLongPtrW, SetWindowPos, ShowWindow, GWL_EXSTYLE, HWND_TOPMOST, SWP_NOACTIVATE,
        SWP_NOSIZE, SWP_NOZORDER, SW_HIDE, SW_SHOWNOACTIVATE, WDA_EXCLUDEFROMCAPTURE, WDA_NONE,
        WS_EX_NOACTIVATE,
    };

    fn hwnd_of(window: &tauri::WebviewWindow) -> Option<HWND> {
        window.hwnd().ok().map(|raw| HWND(raw.0))
    }

    fn rect(r: RECT) -> Rect {
        Rect { left: r.left, top: r.top, right: r.right, bottom: r.bottom }
    }

    unsafe fn work_area(monitor: HMONITOR) -> Option<Rect> {
        let mut info = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        // rcWork excludes the taskbar, wherever it is docked.
        unsafe { GetMonitorInfoW(monitor, &mut info) }
            .as_bool()
            .then(|| rect(info.rcWork))
    }

    pub fn place_and_show(window: &tauri::WebviewWindow, saved: Option<SavedPosition>) {
        let Some(hwnd) = hwnd_of(window) else { return };
        unsafe {
            // Never take focus. (WS_EX_TOOLWINDOW is not used: on this undecorated window it
            // brings back a small native title bar. skipTaskbar already hides it from the taskbar.)
            let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
            SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex_style | WS_EX_NOACTIVATE.0 as isize);

            // Leave the card out of screen sharing and recordings: if one ever shows during a
            // call, only the user sees it. Development builds can turn this off to take
            // screenshots (MOOVY_ALLOW_CAPTURE=1).
            let allow_capture = cfg!(debug_assertions) && std::env::var("MOOVY_ALLOW_CAPTURE").is_ok_and(|v| v == "1");
            let affinity = if allow_capture { WDA_NONE } else { WDA_EXCLUDEFROMCAPTURE };
            let _ = SetWindowDisplayAffinity(hwnd, affinity);

            // The monitor holding the window the user is working in.
            let monitor = MonitorFromWindow(GetForegroundWindow(), MONITOR_DEFAULTTOPRIMARY);
            let Some(work) = work_area(monitor) else { return };

            let (mut dpi_x, mut dpi_y) = (96u32, 96u32);
            let _ = GetDpiForMonitor(monitor, MDT_EFFECTIVE_DPI, &mut dpi_x, &mut dpi_y);
            let scale = dpi_x as f64 / 96.0;

            // Keep the height the card last asked for, converted to this monitor's scale.
            let mut current = RECT::default();
            let _ = GetWindowRect(hwnd, &mut current);
            let current_scale = GetDpiForWindow(hwnd).max(1) as f64 / 96.0;
            let logical_height = if current.bottom > current.top {
                (current.bottom - current.top) as f64 / current_scale
            } else {
                CARD_HEIGHT
            };

            let width = (CARD_WIDTH * scale).round() as i32;
            let height = (logical_height * scale).round() as i32;
            let (x, y) = placement(work, scale, width, height, saved);

            // First move onto the target monitor. If its scaling differs, Windows resizes the
            // window for the new DPI during this move. Then set the exact rectangle, which is
            // now on the same monitor and is not rescaled again.
            let _ = SetWindowPos(hwnd, Some(HWND_TOPMOST), x, y, 0, 0, SWP_NOACTIVATE | SWP_NOSIZE);
            let _ = SetWindowPos(hwnd, Some(HWND_TOPMOST), x, y, width, height, SWP_NOACTIVATE);
            let _ = ShowWindow(hwnd, SW_SHOWNOACTIVATE);

            #[cfg(debug_assertions)]
            println!(
                "[moovy] card placed: work=({},{},{},{}) scale={scale} at=({x},{y}) size={width}x{height} saved={saved:?}",
                work.left, work.top, work.right, work.bottom
            );
        }
    }

    pub fn hide(window: &tauri::WebviewWindow) {
        if let Some(hwnd) = hwnd_of(window) {
            unsafe {
                let _ = ShowWindow(hwnd, SW_HIDE);
            }
        }
    }

    pub fn resize_keep_bottom(window: &tauri::WebviewWindow, logical_height: f64) {
        let Some(hwnd) = hwnd_of(window) else { return };
        unsafe {
            let mut r = RECT::default();
            if GetWindowRect(hwnd, &mut r).is_err() {
                return;
            }
            let scale = GetDpiForWindow(hwnd).max(1) as f64 / 96.0;
            let height = (logical_height * scale).round() as i32;
            if height == r.bottom - r.top {
                return;
            }
            let _ = SetWindowPos(
                hwnd,
                None,
                r.left,
                r.bottom - height,
                r.right - r.left,
                height,
                SWP_NOACTIVATE | SWP_NOZORDER,
            );
        }
    }

    pub fn current_position(window: &tauri::WebviewWindow) -> Option<SavedPosition> {
        let hwnd = hwnd_of(window)?;
        unsafe {
            let mut r = RECT::default();
            GetWindowRect(hwnd, &mut r).ok()?;
            let work = work_area(MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST))?;
            Some(to_saved(work, rect(r)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WORK: Rect = Rect { left: 0, top: 0, right: 1920, bottom: 1032 };

    #[test]
    fn default_is_bottom_left_with_margin() {
        assert_eq!(placement(WORK, 1.0, 380, 200, None), (16, 816));
    }

    #[test]
    fn default_scales_margin_on_high_dpi() {
        let work = Rect { left: 1920, top: 4, right: 3840, bottom: 1132 };
        assert_eq!(placement(work, 1.5, 570, 300, None), (1944, 808));
    }

    #[test]
    fn saved_position_round_trips() {
        let window = Rect { left: 1200, top: 300, right: 1580, bottom: 500 };
        let saved = to_saved(WORK, window);
        assert_eq!(placement(WORK, 1.0, 380, 200, Some(saved)), (1200, 300));
    }

    #[test]
    fn saved_position_keeps_bottom_edge_when_height_changes() {
        let window = Rect { left: 1200, top: 300, right: 1580, bottom: 500 };
        let saved = to_saved(WORK, window);
        // A taller card grows upward from the same bottom edge.
        assert_eq!(placement(WORK, 1.0, 380, 320, Some(saved)), (1200, 180));
    }

    #[test]
    fn saved_position_is_clamped_inside_a_smaller_monitor() {
        // Parked at the far bottom-right of a big screen...
        let saved = SavedPosition { fx: 0.99, fy: 1.0 };
        // ...then shown on a small one: stays fully visible.
        let small = Rect { left: 0, top: 0, right: 1280, bottom: 752 };
        assert_eq!(placement(small, 1.0, 380, 200, Some(saved)), (900, 552));
    }

    #[test]
    fn saved_position_applies_to_the_other_monitor_proportionally() {
        let saved = SavedPosition { fx: 0.5, fy: 0.5 };
        let second = Rect { left: 1920, top: 4, right: 3840, bottom: 1132 };
        assert_eq!(placement(second, 1.5, 570, 300, Some(saved)), (2880, 268));
    }

    #[test]
    fn payload_matches_the_frontend_shape() {
        let payload = CardPayload::Break { movement_id: "wall-angels".into() };
        assert_eq!(
            serde_json::to_value(&payload).unwrap(),
            serde_json::json!({ "kind": "break", "movementId": "wall-angels" })
        );
        assert_eq!(serde_json::to_value(CardPayload::Water).unwrap(), serde_json::json!({ "kind": "water" }));
    }
}
