//! How often cards come: a preset, or custom minutes (decided 2026-10-20). Ids and minutes must
//! match `src/content/paces.ts`.

use serde::Serialize;

use crate::scheduler::MINUTE;

/// Water: off, or 20 to 120 minutes. Move: 30 to 120 minutes. Steps of 5.
pub const WATER_RANGE: (u32, u32) = (20, 120);
pub const MOVE_RANGE: (u32, u32) = (30, 120);
const STEP: u32 = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Pace {
    pub id: &'static str,
    /// `None`: no water.
    pub water_minutes: Option<u32>,
    pub move_minutes: u32,
}

pub const RECOMMENDED: Pace = Pace { id: "recommended", water_minutes: Some(45), move_minutes: 60 };
pub const ACTIVE: Pace = Pace { id: "active", water_minutes: Some(30), move_minutes: 45 };

impl Pace {
    /// A preset by id, or custom minutes (kept inside the allowed ranges, on 5-minute steps).
    pub fn new(id: &str, water_minutes: Option<u32>, move_minutes: u32) -> Self {
        match id {
            "recommended" => RECOMMENDED,
            "active" => ACTIVE,
            _ => Pace {
                id: "custom",
                water_minutes: water_minutes.map(|m| snap(m, WATER_RANGE)),
                move_minutes: snap(move_minutes, MOVE_RANGE),
            },
        }
    }

    /// Settings saved by older versions: "recommended" (then 30/60) becomes today's
    /// Recommended; "relaxed" and "moves-only" keep their minutes as custom.
    pub fn from_old(id: &str) -> Self {
        match id {
            "relaxed" => Pace::new("custom", Some(45), 90),
            "moves-only" => Pace::new("custom", None, 60),
            "active" => ACTIVE,
            _ => RECOMMENDED,
        }
    }

    /// (water_every, move_every) in ms, for the scheduler.
    pub fn timing(self) -> (Option<u64>, u64) {
        (
            self.water_minutes.map(|m| m as u64 * MINUTE),
            self.move_minutes as u64 * MINUTE,
        )
    }
}

fn snap(minutes: u32, (min, max): (u32, u32)) -> u32 {
    (minutes.clamp(min, max) + STEP / 2) / STEP * STEP
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presets_ignore_the_minutes_passed_in() {
        assert_eq!(Pace::new("recommended", Some(99), 99), RECOMMENDED);
        assert_eq!(Pace::new("active", None, 30), ACTIVE);
    }

    #[test]
    fn custom_minutes_snap_to_the_allowed_range_and_steps() {
        let p = Pace::new("custom", Some(7), 500);
        assert_eq!((p.id, p.water_minutes, p.move_minutes), ("custom", Some(20), 120));
        assert_eq!(Pace::new("custom", Some(47), 61).water_minutes, Some(45));
        assert_eq!(Pace::new("custom", None, 60).water_minutes, None);
    }

    #[test]
    fn old_settings_keep_their_meaning() {
        assert_eq!(Pace::from_old("relaxed"), Pace::new("custom", Some(45), 90));
        assert_eq!(Pace::from_old("moves-only").water_minutes, None);
        assert_eq!(Pace::from_old("recommended"), RECOMMENDED);
    }

    #[test]
    fn timing_is_in_milliseconds() {
        assert_eq!(RECOMMENDED.timing(), (Some(45 * MINUTE), 60 * MINUTE));
        assert_eq!(Pace::new("custom", None, 90).timing(), (None, 90 * MINUTE));
    }
}
