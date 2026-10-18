//! Connects the pure scheduler to the real world: reads Windows signals once a second, keeps the
//! scheduler's clock, shows the cards it asks for, and keeps the tray status line current.
//!
//! The scheduler's clock is "Moovy time": real time multiplied by the speed. At speed 1 it is
//! ordinary milliseconds. In fast mode (development) one real second is one Moovy minute, so a
//! full hour-long cycle plays in a minute.

use std::sync::Mutex;
use std::time::{Duration, SystemTime};

use chrono::Local;
use tauri::{menu::MenuItem, AppHandle, Manager, Wry};

use crate::activity::{self, Signals};
use crate::card;
use crate::scheduler::{Card, Config, Decision, Event, Scheduler, Status};

/// A wall-clock gap this long between two ticks means the computer was asleep.
const SLEEP_GAP: Duration = Duration::from_secs(30);
pub const FAST_SPEED: u64 = 60;

pub struct Engine {
    scheduler: Scheduler,
    /// Moovy time, in ms.
    now: u64,
    speed: u64,
    signals: Signals,
    status_text: String,
}

pub type EngineState = Mutex<Engine>;

/// The tray status line, which follows the scheduler.
pub struct TrayItems {
    pub status: MenuItem<Wry>,
}

pub fn init(app: &AppHandle, water_enabled: bool) {
    let speed = std::env::var("MOOVY_SPEED")
        .ok()
        .filter(|_| cfg!(debug_assertions))
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
    app.manage(Mutex::new(Engine {
        scheduler: Scheduler::new(Config::default(), water_enabled),
        now: 0,
        speed,
        signals: Signals::default(),
        status_text: String::new(),
    }));

    let handle = app.clone();
    std::thread::spawn(move || {
        let mut last = SystemTime::now();
        loop {
            std::thread::sleep(Duration::from_secs(1));
            let wall = SystemTime::now();
            let gap = wall.duration_since(last).unwrap_or_default();
            last = wall;
            tick(&handle, gap);
        }
    });
}

fn tick(app: &AppHandle, gap: Duration) {
    let signals = activity::read();
    let decisions = {
        let state = app.state::<EngineState>();
        let mut e = state.lock().unwrap();
        let speed = e.speed;
        e.now += gap.as_millis() as u64 * speed;
        let now = e.now;

        let mut events = Vec::new();
        if gap > SLEEP_GAP {
            events.push(Event::Resumed { now });
        }
        if signals.locked != e.signals.locked {
            events.push(if signals.locked { Event::Locked { now } } else { Event::Unlocked { now } });
        }
        if signals.busy != e.signals.busy {
            events.push(Event::Busy(signals.busy));
        }
        if signals.mic_in_use != e.signals.mic_in_use {
            events.push(Event::MicInUse(signals.mic_in_use));
        }
        events.push(Event::Tick { now, idle_ms: signals.idle_ms * speed });
        e.signals = signals;

        events.into_iter().flat_map(|ev| e.scheduler.handle(ev)).collect::<Vec<_>>()
    };
    dispatch(app, decisions);
    refresh_tray(app);
}

fn dispatch(app: &AppHandle, decisions: Vec<Decision>) {
    for Decision::Show(card) in decisions {
        #[cfg(debug_assertions)]
        println!("[moovy] scheduler shows {card:?}");
        let handle = app.clone();
        let _ = app.run_on_main_thread(move || match card {
            Card::Water => card::show(&handle, card::CardPayload::Water),
            Card::Break => card::show_next_break(&handle),
            Card::Return => card::present_last(&handle),
        });
    }
}

fn send(app: &AppHandle, make: impl FnOnce(u64) -> Event) {
    let decisions = {
        let state = app.state::<EngineState>();
        let mut e = state.lock().unwrap();
        let now = e.now;
        e.scheduler.handle(make(now))
    };
    dispatch(app, decisions);
    refresh_tray(app);
}

pub fn card_closed(app: &AppHandle) {
    send(app, |now| Event::CardClosed { now });
}

pub fn snoozed(app: &AppHandle) {
    send(app, |now| Event::Snoozed { now });
}

/// Show a card by hand (development). The scheduler tracks it like any other.
#[cfg_attr(not(debug_assertions), allow(dead_code))]
pub fn force_show(app: &AppHandle, card: Card) {
    send(app, |_| Event::ForceShow { card });
}

pub fn set_water(app: &AppHandle, enabled: bool) {
    send(app, |_| Event::WaterEnabled(enabled));
}

pub fn speed(app: &AppHandle) -> u64 {
    app.state::<EngineState>().lock().unwrap().speed
}

pub fn set_speed(app: &AppHandle, speed: u64) {
    app.state::<EngineState>().lock().unwrap().speed = speed.max(1);
    refresh_tray(app);
}

fn status_text(e: &Engine) -> String {
    let fast = if e.speed > 1 { " (fast mode)" } else { "" };
    match e.scheduler.status() {
        Status::Next { card, minutes } => {
            let what = if card == Card::Water { "water" } else { "a move" };
            format!("Next: {what} in {minutes} min{fast}")
        }
        Status::Paused { until } => {
            let real_ms = until.saturating_sub(e.now) / e.speed;
            let at = Local::now() + chrono::Duration::milliseconds(real_ms as i64);
            format!("Paused until {}", at.format("%H:%M"))
        }
        Status::Waiting => format!("A card is on its way{fast}"),
    }
}

fn refresh_tray(app: &AppHandle) {
    let Some(items) = app.try_state::<TrayItems>() else { return };
    let (text, changed) = {
        let state = app.state::<EngineState>();
        let mut e = state.lock().unwrap();
        let text = status_text(&e);
        let changed = text != e.status_text;
        e.status_text = text.clone();
        (text, changed)
    };
    if changed {
        #[cfg(debug_assertions)]
        println!("[moovy] tray: {text}");
        let _ = items.status.set_text(text);
    }
}
