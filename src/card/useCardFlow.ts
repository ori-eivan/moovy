import { useCallback, useEffect, useRef, useState } from "react";
import { MOVEMENTS, type Movement } from "../content/movements";

/** What Rust sends with "card:show". Matches `CardPayload` in card.rs. */
export type Payload =
  | { kind: "break"; movementId: string }
  | { kind: "water" }
  | { kind: "hello" }
  | { kind: "updatesNotice" }
  | { kind: "updateReady"; version: string };

export type View =
  | { name: "breakIntro"; movement: Movement }
  | { name: "movement"; movement: Movement; howTo: boolean }
  | { name: "running"; movement: Movement; endsAt: number; durationMs: number }
  | { name: "celebration" }
  | { name: "water" }
  | { name: "waterDone" }
  | { name: "waiting" }
  /** After onboarding: "drag me anywhere". `saved` once the user has let go of it. */
  | { name: "hello"; saved: boolean }
  /** Once, on installs from before 0.1.7: keep checking for updates? */
  | { name: "updatesNotice" }
  /** A new version is downloaded and checked: install now or later? */
  | { name: "updateReady"; version: string };

export const BREAK_INTRO_MS = 3000;
export const CELEBRATION_MS = 4000;
export const WATER_DONE_MS = 1200;
export const WAITING_MS = 2000;
export const SWITCH_SIDES_MS = 3000;
/** How long "Saved!" stays before the hello card leaves. */
export const HELLO_SAVED_MS = 2000;

/** Where the card's decisions go: Rust in the app, a fake in the gallery. */
export interface CardOutcomes {
  started?: () => void;
  done: (kind: "break" | "water") => void;
  skip: () => void;
  snooze: () => void;
  /** The hello card is finished (dragged into place, or closed). */
  helloDone?: () => void;
  updatesAnswer?: (enabled: boolean) => void;
  updateInstall?: () => void;
  updateLater?: () => void;
}

export interface CardActions {
  start: (movement: Movement) => void;
  /** Rep-based movement finished: no timer, the card just closes. */
  repsDone: () => void;
  snooze: () => void;
  waterDone: () => void;
  toggleHowTo: () => void;
  skip: () => void;
  /** The user let go of the hello card after dragging it. */
  helloSaved: () => void;
  helloDone: () => void;
  updatesAnswer: (enabled: boolean) => void;
  updateInstall: () => void;
  updateLater: () => void;
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
      if (payload.kind === "hello") {
        setView({ name: "hello", saved: false });
        return;
      }
      if (payload.kind === "updatesNotice") {
        setView({ name: "updatesNotice" });
        return;
      }
      if (payload.kind === "updateReady") {
        setView({ name: "updateReady", version: payload.version });
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
    repsDone: () => {
      clearTimers();
      // Counted as started and completed in one go.
      outcomesRef.current.started?.();
      outcomesRef.current.done("break");
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
    helloSaved: () => {
      clearTimers();
      setView({ name: "hello", saved: true });
      after(HELLO_SAVED_MS, () => outcomesRef.current.helloDone?.());
    },
    helloDone: () => {
      clearTimers();
      outcomesRef.current.helloDone?.();
    },
    updatesAnswer: (enabled) => {
      clearTimers();
      outcomesRef.current.updatesAnswer?.(enabled);
    },
    updateInstall: () => {
      clearTimers();
      outcomesRef.current.updateInstall?.();
    },
    updateLater: () => {
      clearTimers();
      outcomesRef.current.updateLater?.();
    },
  };

  return { view, now, show, actions };
}
