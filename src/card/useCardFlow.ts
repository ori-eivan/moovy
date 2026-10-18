import { useCallback, useEffect, useRef, useState } from "react";
import { MOVEMENTS, type Movement } from "../content/movements";

/** What Rust sends with "card:show". Matches `CardPayload` in card.rs. */
export type Payload = { kind: "break"; movementId: string } | { kind: "water" };

export type View =
  | { name: "breakIntro"; movement: Movement }
  | { name: "movement"; movement: Movement; howTo: boolean }
  | { name: "running"; movement: Movement; endsAt: number; durationMs: number }
  | { name: "celebration" }
  | { name: "water" }
  | { name: "waterDone" }
  | { name: "waiting" };

export const BREAK_INTRO_MS = 3000;
export const CELEBRATION_MS = 4000;
export const WATER_DONE_MS = 1200;
export const WAITING_MS = 2000;
export const SWITCH_SIDES_MS = 3000;

/** Where the card's decisions go: Rust in the app, a fake in the gallery. */
export interface CardOutcomes {
  started?: () => void;
  done: (kind: "break" | "water") => void;
  skip: () => void;
  snooze: () => void;
}

export interface CardActions {
  start: (movement: Movement) => void;
  snooze: () => void;
  waterDone: () => void;
  toggleHowTo: () => void;
  skip: () => void;
}

/**
 * The card's state machine, shared by the real card and the gallery:
 * break intro → movement → running → celebration, water → done, and snooze → waiting.
 */
export function useCardFlow(outcomes: CardOutcomes, options: { timeScale?: number } = {}) {
  const timeScale = options.timeScale ?? 1;
  const [view, setView] = useState<View>({ name: "water" });
  const [now, setNow] = useState(() => Date.now());
  const timers = useRef<number[]>([]);
  const outcomesRef = useRef(outcomes);
  outcomesRef.current = outcomes;

  const clearTimers = useCallback(() => {
    timers.current.forEach((id) => window.clearTimeout(id));
    timers.current = [];
  }, []);

  const after = useCallback((ms: number, fn: () => void) => {
    timers.current.push(window.setTimeout(fn, ms));
  }, []);

  useEffect(() => clearTimers, [clearTimers]);

  /** A new card arrives and resets everything. */
  const show = useCallback(
    (payload: Payload) => {
      clearTimers();
      if (payload.kind === "water") {
        setView({ name: "water" });
        return;
      }
      const movement = MOVEMENTS[payload.movementId];
      if (!movement) return;
      setView({ name: "breakIntro", movement });
      after(BREAK_INTRO_MS, () => setView({ name: "movement", movement, howTo: false }));
    },
    [after, clearTimers],
  );

  // Countdown tick while a movement runs.
  useEffect(() => {
    if (view.name !== "running") return;
    const id = window.setInterval(() => setNow(Date.now()), 250);
    return () => window.clearInterval(id);
  }, [view.name]);

  // When the countdown reaches zero: celebrate, then report the break as done.
  useEffect(() => {
    if (view.name === "running" && now >= view.endsAt) {
      clearTimers();
      setView({ name: "celebration" });
      after(CELEBRATION_MS, () => outcomesRef.current.done("break"));
    }
  }, [view, now, after, clearTimers]);

  const actions: CardActions = {
    start: (movement) => {
      clearTimers();
      const startedAt = Date.now();
      const durationMs = movement.seconds * 1000 * timeScale;
      setNow(startedAt);
      setView({ name: "running", movement, endsAt: startedAt + durationMs, durationMs });
      outcomesRef.current.started?.();
    },
    snooze: () => {
      clearTimers();
      setView({ name: "waiting" });
      after(WAITING_MS, () => outcomesRef.current.snooze());
    },
    waterDone: () => {
      clearTimers();
      setView({ name: "waterDone" });
      after(WATER_DONE_MS, () => outcomesRef.current.done("water"));
    },
    toggleHowTo: () => setView((v) => (v.name === "movement" ? { ...v, howTo: !v.howTo } : v)),
    skip: () => {
      clearTimers();
      outcomesRef.current.skip();
    },
  };

  return { view, now, show, actions };
}
