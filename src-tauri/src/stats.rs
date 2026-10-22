//! What happens to the cards, kept only on this computer and never sent anywhere.
//!
//! - A count per day. Testers copy a one-line summary from the tray ("Copy my stats") and share
//!   it themselves; that number is the 50% release gate in the PRD.
//! - A log of each card that was done or closed with X, shown day by day in the Stats tab.
//!   Snoozes and cards left unanswered are not in the log.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

const STORE_FILE: &str = "moovy.json";
const KEY_DAYS: &str = "days";
const KEY_LOG: &str = "log";
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

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Outcome {
    Done,
    Skipped,
}

/// One card in the day log.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    /// Local time the card was answered, "HH:MM".
    pub time: String,
    /// "break" or "water".
    pub kind: String,
    /// The movement on a break card. The page turns it into a name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub movement_id: Option<String>,
    pub outcome: Outcome,
}

type Log = BTreeMap<String, Vec<Entry>>;

/// One day of the log, as the Stats tab gets it.
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct DayLog {
    pub date: String,
    pub entries: Vec<Entry>,
}

fn today() -> String {
    chrono::Local::now().date_naive().to_string()
}

/// Keeps the last 30 days only. Dates sort correctly as YYYY-MM-DD strings.
fn keep_recent<T>(days: &mut BTreeMap<String, T>) {
    while days.len() > KEEP_DAYS {
        days.pop_first();
    }
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
    keep_recent(&mut days);
    if let Ok(value) = serde_json::to_value(&days) {
        store.set(KEY_DAYS, value);
        let _ = store.save();
    }
}

fn load_log(app: &AppHandle) -> Log {
    app.store(STORE_FILE)
        .ok()
        .and_then(|s| s.get(KEY_LOG))
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default()
}

/// Adds a done or skipped card to today's log.
pub fn log_card(app: &AppHandle, kind: &str, movement_id: Option<String>, outcome: Outcome) {
    let Ok(store) = app.store(STORE_FILE) else { return };
    let now = chrono::Local::now();
    let entry = Entry { time: now.format("%H:%M").to_string(), kind: kind.to_string(), movement_id, outcome };
    let mut log = load_log(app);
    add_entry(&mut log, now.date_naive().to_string(), entry);
    if let Ok(value) = serde_json::to_value(&log) {
        store.set(KEY_LOG, value);
        let _ = store.save();
    }
}

fn add_entry(log: &mut Log, date: String, entry: Entry) {
    log.entry(date).or_default().push(entry);
    keep_recent(log);
}

/// Every day in the log, newest first. Days with no entries are left out.
pub fn day_logs(app: &AppHandle) -> Vec<DayLog> {
    newest_first(load_log(app))
}

fn newest_first(log: Log) -> Vec<DayLog> {
    log.into_iter()
        .rev()
        .filter(|(_, entries)| !entries.is_empty())
        .map(|(date, entries)| DayLog { date, entries })
        .collect()
}

pub fn clear(app: &AppHandle) {
    if let Ok(store) = app.store(STORE_FILE) {
        store.delete(KEY_DAYS);
        store.delete(KEY_LOG);
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

    fn entry(time: &str, outcome: Outcome) -> Entry {
        Entry { time: time.into(), kind: "water".into(), movement_id: None, outcome }
    }

    #[test]
    fn log_keeps_the_last_thirty_days() {
        let mut log = Log::new();
        for i in 1..=31 {
            add_entry(&mut log, format!("2026-10-{i:02}"), entry("10:00", Outcome::Done));
        }
        assert_eq!(log.len(), 30);
        assert!(!log.contains_key("2026-10-01"));
    }

    #[test]
    fn day_logs_are_newest_first_in_the_order_they_happened() {
        let mut log = Log::new();
        add_entry(&mut log, "2026-10-20".into(), entry("09:00", Outcome::Done));
        add_entry(&mut log, "2026-10-21".into(), entry("09:30", Outcome::Skipped));
        add_entry(&mut log, "2026-10-21".into(), entry("10:00", Outcome::Done));
        let days = newest_first(log);
        assert_eq!(days[0].date, "2026-10-21");
        assert_eq!(days[0].entries[0].time, "09:30");
        assert_eq!(days[1].date, "2026-10-20");
    }

    #[test]
    fn entry_json_matches_the_page() {
        let break_entry = Entry {
            time: "14:05".into(),
            kind: "break".into(),
            movement_id: Some("wall-angels".into()),
            outcome: Outcome::Skipped,
        };
        let json = serde_json::to_string(&break_entry).unwrap();
        assert_eq!(json, r#"{"time":"14:05","kind":"break","movementId":"wall-angels","outcome":"skipped"}"#);
        assert!(!serde_json::to_string(&entry("09:00", Outcome::Done)).unwrap().contains("movementId"));
    }

    #[test]
    fn rate_is_rounded() {
        let mut days = Days::new();
        days.insert("2026-10-18".into(), day(3, 2));
        assert!(summarize(&days, "0.1.0").contains("(67%)"));
    }
}
