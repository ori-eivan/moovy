//! A silent local count of what happens to the cards, per day. Never shown in the UI and never
//! sent anywhere. Testers copy a one-line summary from the tray ("Copy my stats") and share it
//! themselves; that number is the 50% release gate in the PRD.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

const STORE_FILE: &str = "moovy.json";
const KEY_DAYS: &str = "days";
const KEEP_DAYS: usize = 30;

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct Day {
    pub break_shown: u32,
    pub break_started: u32,
    pub break_completed: u32,
    pub water_shown: u32,
    pub water_done: u32,
    pub snoozed: u32,
    pub skipped: u32,
}

#[derive(Clone, Copy, Debug)]
pub enum Event {
    BreakShown,
    BreakStarted,
    BreakCompleted,
    WaterShown,
    WaterDone,
    Snoozed,
    Skipped,
}

impl Day {
    fn apply(&mut self, event: Event) {
        let field = match event {
            Event::BreakShown => &mut self.break_shown,
            Event::BreakStarted => &mut self.break_started,
            Event::BreakCompleted => &mut self.break_completed,
            Event::WaterShown => &mut self.water_shown,
            Event::WaterDone => &mut self.water_done,
            Event::Snoozed => &mut self.snoozed,
            Event::Skipped => &mut self.skipped,
        };
        *field += 1;
    }

    fn add(&mut self, other: &Day) {
        self.break_shown += other.break_shown;
        self.break_started += other.break_started;
        self.break_completed += other.break_completed;
        self.water_shown += other.water_shown;
        self.water_done += other.water_done;
        self.snoozed += other.snoozed;
        self.skipped += other.skipped;
    }
}

type Days = BTreeMap<String, Day>;

fn today() -> String {
    chrono::Local::now().date_naive().to_string()
}

fn load(app: &AppHandle) -> Days {
    app.store(STORE_FILE)
        .ok()
        .and_then(|s| s.get(KEY_DAYS))
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default()
}

pub fn record(app: &AppHandle, event: Event) {
    let Ok(store) = app.store(STORE_FILE) else { return };
    let mut days = load(app);
    days.entry(today()).or_default().apply(event);
    // Keep the last 30 days only. Dates sort correctly as YYYY-MM-DD strings.
    while days.len() > KEEP_DAYS {
        let oldest = days.keys().next().cloned().unwrap();
        days.remove(&oldest);
    }
    if let Ok(value) = serde_json::to_value(&days) {
        store.set(KEY_DAYS, value);
        let _ = store.save();
    }
}

pub fn clear(app: &AppHandle) {
    if let Ok(store) = app.store(STORE_FILE) {
        store.delete(KEY_DAYS);
        let _ = store.save();
    }
}

/// The one line testers copy and send: totals over the last 7 days that have any activity.
pub fn summary(app: &AppHandle) -> String {
    let version = app.package_info().version.to_string();
    summarize(&load(app), &version)
}

pub fn summarize(days: &Days, version: &str) -> String {
    let mut total = Day::default();
    let recent: Vec<&Day> = days.values().rev().take(7).collect();
    for day in &recent {
        total.add(day);
    }
    let start_rate = (total.break_started * 100 + total.break_shown / 2)
        .checked_div(total.break_shown)
        .map_or_else(|| "n/a".to_string(), |rate| format!("{rate}%"));
    format!(
        "Moovy {version}, last {} active days: {} break cards, started {} ({start_rate}), completed {}, snoozed {}, skipped {}; water cards {}, done {}",
        recent.len(),
        total.break_shown,
        total.break_started,
        total.break_completed,
        total.snoozed,
        total.skipped,
        total.water_shown,
        total.water_done,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(shown: u32, started: u32) -> Day {
        Day { break_shown: shown, break_started: started, ..Default::default() }
    }

    #[test]
    fn summary_counts_the_last_seven_active_days() {
        let mut days = Days::new();
        for i in 1..=9 {
            days.insert(format!("2026-10-{i:02}"), day(2, 1));
        }
        let line = summarize(&days, "0.1.0");
        assert!(line.contains("last 7 active days: 14 break cards, started 7 (50%)"), "{line}");
    }

    #[test]
    fn summary_without_cards_has_no_rate() {
        let line = summarize(&Days::new(), "0.1.0");
        assert!(line.contains("0 break cards, started 0 (n/a)"), "{line}");
    }

    #[test]
    fn rate_is_rounded() {
        let mut days = Days::new();
        days.insert("2026-10-18".into(), day(3, 2));
        assert!(summarize(&days, "0.1.0").contains("(67%)"));
    }
}
