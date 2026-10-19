//! When cards appear. Pure logic: no clock, no Windows calls, no Tauri. Everything comes in as
//! events with timestamps, and decisions come out. That is what makes it testable against a
//! simulated clock (see the tests at the bottom).
//!
//! The rules (PRD §7): a 60-minute cycle of *active* computer time, water at :30, a break at
//! :60. Idle pauses the count, 15 minutes away restarts the clock (keeping the water/break
//! order). Never two cards within 15 minutes,
//! never while busy, in a call or paused, and a card that waits 20 minutes is dropped.

pub const MINUTE: u64 = 60_000;

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub cycle: u64,
    pub water_at: u64,
    pub idle_pause_after: u64,
    pub away_reset_after: u64,
    pub min_gap: u64,
    pub snooze: u64,
    pub max_wait: u64,
    pub grace_after_unlock: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cycle: 60 * MINUTE,
            water_at: 30 * MINUTE,
            idle_pause_after: 2 * MINUTE,
            away_reset_after: 15 * MINUTE,
            min_gap: 15 * MINUTE,
            snooze: 10 * MINUTE,
            max_wait: 20 * MINUTE,
            grace_after_unlock: 2 * MINUTE,
        }
    }
}

/// How often cards come, picked in onboarding and settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pace {
    /// Water at :30, a move at :60.
    Recommended,
    /// Water at :45, a move at :90.
    Relaxed,
    /// A move every 60 minutes, no water.
    MovesOnly,
}

impl Pace {
    pub fn from_id(id: &str) -> Self {
        match id {
            "relaxed" => Pace::Relaxed,
            "moves-only" => Pace::MovesOnly,
            _ => Pace::Recommended,
        }
    }

    pub fn id(self) -> &'static str {
        match self {
            Pace::Recommended => "recommended",
            Pace::Relaxed => "relaxed",
            Pace::MovesOnly => "moves-only",
        }
    }

    pub fn water(self) -> bool {
        self != Pace::MovesOnly
    }

    /// (cycle, water_at), in ms of active time.
    pub fn timing(self) -> (u64, u64) {
        match self {
            Pace::Relaxed => (90 * MINUTE, 45 * MINUTE),
            Pace::Recommended | Pace::MovesOnly => (60 * MINUTE, 30 * MINUTE),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Card {
    Water,
    Break,
    /// The card that was snoozed, coming back.
    Return,
}

#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// Once a second. `idle_ms`: time since the last keyboard or mouse input.
    Tick { now: u64, idle_ms: u64 },
    /// Screen locked (or screen saver).
    Locked { now: u64 },
    Unlocked { now: u64 },
    /// Back from sleep.
    Resumed { now: u64 },
    /// Presentation, full-screen game, quiet time.
    Busy(bool),
    MicInUse(bool),
    /// The user closed the open card (done or skipped: both end it the same way).
    CardClosed { now: u64 },
    /// A card was put on screen by hand (tray, testing). It is tracked like any other card.
    ForceShow { card: Card },
    /// The user snoozed the open card.
    Snoozed { now: u64 },
    // Pause and skip are tested and ready, but not in the v0.1 tray (removed 2026-10-18).
    #[allow(dead_code)]
    PauseUntil(u64),
    #[allow(dead_code)]
    Resume,
    #[allow(dead_code)]
    SkipNext,
    // Superseded by Pace in the app (2026-10-19); kept for the scheduler tests.
    #[allow(dead_code)]
    WaterEnabled(bool),
    /// A new pace: cycle length, when water comes, and whether it comes at all.
    Pace(Pace),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decision {
    Show(Card),
}

/// What the tray says.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    /// Minutes of active time until the next card.
    Next { card: Card, minutes: u64 },
    Paused { until: u64 },
    Waiting,
}

#[derive(Clone, Copy, Debug)]
struct Queued {
    card: Card,
    since: u64,
}

#[derive(Debug)]
pub struct Scheduler {
    config: Config,
    /// Active time in the current cycle.
    active: u64,
    water_done_this_cycle: bool,
    water_enabled: bool,
    last_tick: Option<u64>,
    /// Set once the user has been away long enough to reset, so it resets only once.
    away: bool,
    queued: Option<Queued>,
    card_open: Option<Card>,
    /// When the last card closed. The minimum gap is measured from here.
    last_card_at: Option<u64>,
    snoozed: Option<(Card, u64)>,
    /// What the current or last Return card really is (water or break).
    last_snoozed_kind: Option<Card>,
    paused_until: Option<u64>,
    skip_next: bool,
    busy: bool,
    mic: bool,
    locked: bool,
    grace_until: Option<u64>,
}

impl Scheduler {
    pub fn new(config: Config, water_enabled: bool) -> Self {
        Self {
            config,
            active: 0,
            water_done_this_cycle: false,
            water_enabled,
            last_tick: None,
            away: false,
            queued: None,
            card_open: None,
            last_card_at: None,
            snoozed: None,
            last_snoozed_kind: None,
            paused_until: None,
            skip_next: false,
            busy: false,
            mic: false,
            locked: false,
            grace_until: None,
        }
    }

    fn restart_cycle(&mut self) {
        self.active = 0;
        self.water_done_this_cycle = false;
    }

    /// Being away (locked, asleep, 15 minutes idle) counts as a break: the clock starts over, but
    /// the order is kept. If water already came this cycle, the next card is still the break,
    /// 30 active minutes after coming back, so frequent breaks don't turn into water only.
    fn away_reset(&mut self) {
        let break_is_next = self.water_enabled && self.water_done_this_cycle;
        self.restart_cycle();
        if break_is_next {
            self.active = self.config.cycle - self.config.water_at;
            self.water_done_this_cycle = true;
        }
        self.queued = None;
        self.snoozed = None;
    }

    pub fn handle(&mut self, event: Event) -> Vec<Decision> {
        match event {
            Event::Tick { now, idle_ms } => return self.tick(now, idle_ms),
            Event::Locked { now } => {
                self.locked = true;
                self.last_tick = Some(now);
                self.away_reset();
            }
            Event::Unlocked { now } => {
                self.locked = false;
                self.last_tick = Some(now);
                self.grace_until = Some(now + self.config.grace_after_unlock);
            }
            Event::Resumed { now } => {
                self.last_tick = Some(now);
                self.away_reset();
                self.grace_until = Some(now + self.config.grace_after_unlock);
            }
            Event::Busy(busy) => self.busy = busy,
            Event::MicInUse(mic) => self.mic = mic,
            Event::CardClosed { now } => {
                // Done or skipped, a closed break starts a new cycle.
                if let Some(card) = self.card_open.take() {
                    self.last_card_at = Some(now);
                    let real = if card == Card::Return { self.snoozed_kind() } else { card };
                    if real == Card::Break {
                        self.restart_cycle();
                    }
                }
            }
            Event::ForceShow { card } => {
                self.queued = None;
                self.card_open = Some(card);
                return vec![Decision::Show(card)];
            }
            Event::Snoozed { now } => {
                if let Some(card) = self.card_open.take() {
                    let kind = if card == Card::Return { self.snoozed_kind() } else { card };
                    self.snoozed = Some((kind, now + self.config.snooze));
                }
                self.last_card_at = Some(now);
            }
            Event::PauseUntil(until) => {
                self.paused_until = Some(until);
                self.queued = None;
            }
            Event::Resume => self.paused_until = None,
            Event::SkipNext => self.skip_next = true,
            Event::WaterEnabled(enabled) => self.set_water(enabled),
            Event::Pace(pace) => {
                let (cycle, water_at) = pace.timing();
                self.config.cycle = cycle;
                self.config.water_at = water_at;
                self.set_water(pace.water());
            }
        }
        Vec::new()
    }

    fn set_water(&mut self, enabled: bool) {
        self.water_enabled = enabled;
        if !enabled && matches!(self.queued, Some(Queued { card: Card::Water, .. })) {
            self.queued = None;
        }
    }

    /// Which card a Return brings back.
    fn snoozed_kind(&self) -> Card {
        self.last_snoozed_kind.unwrap_or(Card::Break)
    }

    fn tick(&mut self, now: u64, idle_ms: u64) -> Vec<Decision> {
        let dt = self.last_tick.map_or(0, |last| now.saturating_sub(last));
        self.last_tick = Some(now);

        if let Some(until) = self.paused_until {
            if now >= until {
                self.paused_until = None;
            } else {
                return Vec::new();
            }
        }
        if self.locked {
            return Vec::new();
        }

        // Count active time. Idle pauses the count; long idle resets the cycle once.
        if idle_ms >= self.config.away_reset_after {
            if !self.away {
                self.away = true;
                self.away_reset();
            }
            return Vec::new();
        }
        self.away = false;
        if idle_ms < self.config.idle_pause_after && self.card_open.is_none() {
            self.active += dt;
        }

        // Something became due?
        if self.water_enabled
            && !self.water_done_this_cycle
            && self.active >= self.config.water_at
            && self.queued.is_none()
            && self.card_open != Some(Card::Water)
        {
            self.water_done_this_cycle = true;
            self.enqueue(Card::Water, now);
        }
        let break_pending = matches!(self.queued, Some(Queued { card: Card::Break, .. }))
            || matches!(self.snoozed, Some((Card::Break, _)))
            || (matches!(self.queued, Some(Queued { card: Card::Return, .. })) && self.last_snoozed_kind == Some(Card::Break));
        if self.active >= self.config.cycle && !break_pending && self.card_open.is_none() {
            // The break outranks a waiting water card.
            self.water_done_this_cycle = true;
            self.enqueue(Card::Break, now);
        }
        if let Some((card, due)) = self.snoozed
            && now >= due
            && self.queued.is_none()
        {
            self.snoozed = None;
            self.last_snoozed_kind = Some(card);
            self.queued = Some(Queued { card: Card::Return, since: now });
        }

        self.try_show(now)
    }

    fn enqueue(&mut self, card: Card, now: u64) {
        if self.skip_next {
            self.skip_next = false;
            if card == Card::Break {
                self.restart_cycle();
            }
            return;
        }
        self.queued = Some(Queued { card, since: now });
    }

    fn try_show(&mut self, now: u64) -> Vec<Decision> {
        let Some(q) = self.queued else { return Vec::new() };

        // Waited too long: drop it rather than show something stale.
        if now.saturating_sub(q.since) > self.config.max_wait {
            self.queued = None;
            if q.card == Card::Break {
                self.restart_cycle();
            }
            return Vec::new();
        }

        let blocked = self.card_open.is_some()
            || self.busy
            || self.mic
            || self.grace_until.is_some_and(|g| now < g)
            || (q.card != Card::Return
                && self.last_card_at.is_some_and(|last| now.saturating_sub(last) < self.config.min_gap));
        if blocked {
            return Vec::new();
        }

        self.queued = None;
        self.card_open = Some(q.card);
        vec![Decision::Show(q.card)]
    }

    pub fn status(&self) -> Status {
        if let Some(until) = self.paused_until {
            return Status::Paused { until };
        }
        if self.queued.is_some() || self.card_open.is_some() {
            return Status::Waiting;
        }
        let to_minutes = |ms: u64| ms.div_ceil(MINUTE);
        if self.water_enabled && !self.water_done_this_cycle && self.active < self.config.water_at {
            return Status::Next { card: Card::Water, minutes: to_minutes(self.config.water_at - self.active) };
        }
        Status::Next { card: Card::Break, minutes: to_minutes(self.config.cycle.saturating_sub(self.active)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SEC: u64 = 1_000;

    /// A simulated computer: advances the clock one second at a time.
    struct Sim {
        s: Scheduler,
        now: u64,
        idle: u64,
        shown: Vec<(u64, Card)>,
    }

    impl Sim {
        fn new() -> Self {
            let mut s = Scheduler::new(Config::default(), true);
            s.handle(Event::Tick { now: 0, idle_ms: 0 });
            Self { s, now: 0, idle: 0, shown: Vec::new() }
        }
        /// Run `ms` of time. `active` = the user keeps typing; otherwise idle grows.
        fn run(&mut self, ms: u64, active: bool) {
            let end = self.now + ms;
            while self.now < end {
                self.now += SEC;
                self.idle = if active { 0 } else { self.idle + SEC };
                for d in self.s.handle(Event::Tick { now: self.now, idle_ms: self.idle }) {
                    let Decision::Show(card) = d;
                    self.shown.push((self.now, card));
                }
            }
        }
        fn work(&mut self, minutes: u64) {
            self.run(minutes * MINUTE, true);
        }
        fn close(&mut self) {
            self.s.handle(Event::CardClosed { now: self.now });
        }
        fn cards(&self) -> Vec<Card> {
            self.shown.iter().map(|(_, c)| *c).collect()
        }
    }

    #[test]
    fn water_at_30_then_break_at_60() {
        let mut sim = Sim::new();
        sim.work(29);
        assert!(sim.shown.is_empty());
        sim.work(1);
        assert_eq!(sim.cards(), vec![Card::Water]);
        sim.close();
        sim.work(30);
        assert_eq!(sim.cards(), vec![Card::Water, Card::Break]);
    }

    #[test]
    fn a_new_cycle_starts_after_the_break() {
        let mut sim = Sim::new();
        sim.work(30);
        sim.close();
        sim.work(30);
        sim.close();
        sim.work(30);
        assert_eq!(sim.cards(), vec![Card::Water, Card::Break, Card::Water]);
    }

    #[test]
    fn reading_without_input_pauses_but_does_not_reset() {
        let mut sim = Sim::new();
        sim.work(20);
        sim.run(10 * MINUTE, false); // reading a long document
        // The first 2 idle minutes still count, the other 8 do not, and nothing reset:
        // just under 22 active minutes, so water in a little over 8.
        assert_eq!(sim.s.status(), Status::Next { card: Card::Water, minutes: 9 });
        sim.work(8);
        assert!(sim.shown.is_empty());
        sim.work(1);
        assert_eq!(sim.cards(), vec![Card::Water]);
    }

    #[test]
    fn fifteen_minutes_away_resets_the_cycle() {
        let mut sim = Sim::new();
        sim.work(25);
        sim.run(16 * MINUTE, false);
        sim.work(25);
        assert!(sim.shown.is_empty(), "the away time reset the cycle");
        sim.work(5);
        assert_eq!(sim.cards(), vec![Card::Water]);
    }

    #[test]
    fn a_lunch_break_after_water_keeps_the_break_next() {
        let mut sim = Sim::new();
        sim.work(30);
        sim.close();
        sim.work(10);
        sim.run(30 * MINUTE, false); // lunch
        assert_eq!(sim.s.status(), Status::Next { card: Card::Break, minutes: 30 });
        sim.work(29);
        assert_eq!(sim.cards(), vec![Card::Water], "no second water after lunch");
        sim.work(1);
        assert_eq!(sim.cards(), vec![Card::Water, Card::Break]);
    }

    #[test]
    fn a_lunch_break_after_the_break_starts_with_water() {
        let mut sim = Sim::new();
        sim.work(30);
        sim.close();
        sim.work(30);
        sim.close();
        sim.work(5);
        sim.s.handle(Event::Locked { now: sim.now });
        sim.now += 30 * MINUTE;
        sim.s.handle(Event::Unlocked { now: sim.now });
        sim.work(30);
        assert_eq!(sim.cards(), vec![Card::Water, Card::Break, Card::Water]);
    }

    #[test]
    fn locking_resets_and_unlock_has_a_grace_period() {
        let mut sim = Sim::new();
        sim.work(25);
        sim.s.handle(Event::Locked { now: sim.now });
        sim.now += 5 * MINUTE;
        sim.s.handle(Event::Unlocked { now: sim.now });
        sim.work(29);
        assert!(sim.shown.is_empty());
        sim.work(1);
        assert_eq!(sim.cards(), vec![Card::Water]);
    }

    #[test]
    fn sleep_drops_everything_without_a_backlog() {
        let mut sim = Sim::new();
        sim.work(31);
        assert_eq!(sim.cards(), vec![Card::Water]);
        sim.close();
        sim.work(28);
        sim.now += 3 * 60 * MINUTE; // laptop asleep for three hours
        sim.s.handle(Event::Resumed { now: sim.now });
        sim.work(2);
        assert_eq!(sim.cards(), vec![Card::Water], "no cards right after waking");
    }

    #[test]
    fn nothing_shows_during_a_call_and_the_card_waits() {
        let mut sim = Sim::new();
        sim.s.handle(Event::MicInUse(true));
        sim.work(35);
        assert!(sim.shown.is_empty());
        sim.s.handle(Event::MicInUse(false));
        sim.work(1);
        assert_eq!(sim.cards(), vec![Card::Water]);
    }

    #[test]
    fn a_card_that_waits_too_long_is_dropped() {
        let mut sim = Sim::new();
        sim.s.handle(Event::Busy(true)); // a long presentation
        sim.work(55);
        sim.s.handle(Event::Busy(false));
        sim.work(1);
        assert!(sim.shown.is_empty(), "water waited 25 minutes and was dropped");
        sim.work(5);
        assert_eq!(sim.cards(), vec![Card::Break]);
    }

    #[test]
    fn snooze_brings_the_same_card_back_after_ten_minutes() {
        let mut sim = Sim::new();
        sim.work(30);
        sim.s.handle(Event::Snoozed { now: sim.now });
        sim.work(9);
        assert_eq!(sim.cards(), vec![Card::Water]);
        sim.work(1);
        assert_eq!(sim.cards(), vec![Card::Water, Card::Return]);
    }

    #[test]
    fn snoozing_the_break_does_not_create_a_second_break() {
        let mut sim = Sim::new();
        sim.work(30);
        sim.close();
        sim.work(30);
        sim.s.handle(Event::Snoozed { now: sim.now });
        sim.work(10);
        assert_eq!(sim.cards(), vec![Card::Water, Card::Break, Card::Return]);
        sim.close();
        sim.work(29);
        assert_eq!(sim.cards().len(), 3, "the returned break started a new cycle");
    }

    #[test]
    fn cards_are_at_least_fifteen_minutes_apart() {
        let mut sim = Sim::new();
        // A call holds the water card from :30 to :45.
        sim.s.handle(Event::MicInUse(true));
        sim.work(45);
        sim.s.handle(Event::MicInUse(false));
        sim.work(1);
        assert_eq!(sim.cards(), vec![Card::Water]);
        sim.close();
        let closed_at = sim.now;
        // The break is due at :60, only 14 minutes later, so it waits for the gap.
        sim.work(14);
        assert_eq!(sim.cards(), vec![Card::Water], "break waits for the gap");
        sim.work(2);
        assert_eq!(sim.cards(), vec![Card::Water, Card::Break]);
        let gap = sim.shown[1].0 - closed_at;
        assert!(gap >= 15 * MINUTE, "gap was {} minutes", gap / MINUTE);
    }

    #[test]
    fn pause_holds_everything_and_freezes_the_count() {
        let mut sim = Sim::new();
        sim.work(20);
        sim.s.handle(Event::PauseUntil(sim.now + 60 * MINUTE));
        sim.work(60);
        assert!(sim.shown.is_empty());
        sim.work(9);
        assert!(sim.shown.is_empty(), "the paused hour did not count");
        sim.work(1);
        assert_eq!(sim.cards(), vec![Card::Water]);
    }

    #[test]
    fn skip_next_skips_one_card() {
        let mut sim = Sim::new();
        sim.s.handle(Event::SkipNext);
        sim.work(30);
        assert!(sim.shown.is_empty());
        sim.work(30);
        assert_eq!(sim.cards(), vec![Card::Break]);
    }

    #[test]
    fn water_off_means_only_breaks() {
        let mut sim = Sim::new();
        sim.s.handle(Event::WaterEnabled(false));
        sim.work(60);
        assert_eq!(sim.cards(), vec![Card::Break]);
    }

    #[test]
    fn relaxed_pace_means_water_at_45_and_a_move_at_90() {
        let mut sim = Sim::new();
        sim.s.handle(Event::Pace(Pace::Relaxed));
        sim.work(44);
        assert!(sim.shown.is_empty());
        sim.work(1);
        assert_eq!(sim.cards(), vec![Card::Water]);
        sim.close();
        sim.work(44);
        assert_eq!(sim.cards(), vec![Card::Water]);
        sim.work(1);
        assert_eq!(sim.cards(), vec![Card::Water, Card::Break]);
    }

    #[test]
    fn moves_only_pace_never_shows_water() {
        let mut sim = Sim::new();
        sim.s.handle(Event::Pace(Pace::MovesOnly));
        sim.work(60);
        assert_eq!(sim.cards(), vec![Card::Break]);
    }

    #[test]
    fn a_lunch_break_on_the_relaxed_pace_keeps_the_order_and_the_spacing() {
        let mut sim = Sim::new();
        sim.s.handle(Event::Pace(Pace::Relaxed));
        sim.work(45);
        sim.close();
        sim.run(30 * MINUTE, false); // lunch
        assert_eq!(sim.s.status(), Status::Next { card: Card::Break, minutes: 45 });
    }

    #[test]
    fn pace_ids_round_trip() {
        for pace in [Pace::Recommended, Pace::Relaxed, Pace::MovesOnly] {
            assert_eq!(Pace::from_id(pace.id()), pace);
        }
        assert_eq!(Pace::from_id("anything else"), Pace::Recommended);
    }

    #[test]
    fn status_counts_down_to_the_next_card() {
        let mut sim = Sim::new();
        sim.work(18);
        assert_eq!(sim.s.status(), Status::Next { card: Card::Water, minutes: 12 });
        sim.work(12);
        sim.close();
        sim.work(10);
        assert_eq!(sim.s.status(), Status::Next { card: Card::Break, minutes: 20 });
    }
}
