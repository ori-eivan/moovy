import { useEffect, useLayoutEffect, useRef, useState } from "react";
import { gsap } from "gsap";
import { MOVEMENTS } from "../content/movements";
import "./onboarding.css";
import "./settings.css";
import "./stats.css";

/** One card in the day log, as Rust sends it (src-tauri/src/stats.rs, `Entry`). */
export interface LogEntry {
  /** Local time, "HH:MM". */
  time: string;
  kind: "break" | "water";
  movementId?: string;
  outcome: "done" | "skipped";
}

export interface DayLog {
  /** "YYYY-MM-DD", local date. */
  date: string;
  entries: LogEntry[];
}

interface Props {
  /** Reads the day log. Called when the screen opens and each time the window comes back. */
  loadDays: () => Promise<DayLog[]>;
  /** Gallery only: start with this day open. */
  openDate?: string;
}

const reducedMotion = () => window.matchMedia("(prefers-reduced-motion: reduce)").matches;

/**
 * The Stats screen, opened from "Stats" in the tray: the last 7 days in one line, then one row
 * per day. Clicking a row slides it up to the top and the day's cards appear under it.
 * Clicking the row again (or Back) returns to the list.
 */
export default function StatsView({ loadDays, openDate }: Props) {
  const [days, setDays] = useState<DayLog[] | null>(null);

  useEffect(() => {
    const load = () => void loadDays().then(setDays);
    load();
    window.addEventListener("focus", load);
    return () => window.removeEventListener("focus", load);
  }, [loadDays]);

  return (
    <main className="settings stats-screen">
      <header>
        <h1>Stats</h1>
      </header>
      {days === null ? null : days.length === 0 ? <Empty /> : <Days days={days} openDate={openDate} />}
    </main>
  );
}

function Empty() {
  return (
    <section className="stats-empty">
      <p className="title">Your days show up here</p>
      <p className="line">Every water break and move you do, or close with ×, is listed by day.</p>
    </section>
  );
}

function Days({ days, openDate }: { days: DayLog[]; openDate?: string }) {
  const [open, setOpen] = useState<string | null>(openDate ?? null);
  const listRef = useRef<HTMLDivElement>(null);
  // Where the clicked row was, relative to the list, so it can slide from there.
  const from = useRef<{ date: string; offset: number; opening: boolean } | null>(null);
  const busy = useRef(false);

  const rowOffset = (date: string) => {
    const list = listRef.current;
    const row = list?.querySelector<HTMLElement>(`[data-date="${date}"]`);
    return list && row ? row.getBoundingClientRect().top - list.getBoundingClientRect().top : 0;
  };

  const openDay = (date: string) => {
    if (busy.current) return;
    if (reducedMotion()) return setOpen(date);
    busy.current = true;
    from.current = { date, offset: rowOffset(date), opening: true };
    const others = listRef.current?.querySelectorAll(`.day-row:not([data-date="${date}"]), .week-summary`) ?? [];
    gsap.to(others, { autoAlpha: 0, duration: 0.15, ease: "power2.in", onComplete: () => setOpen(date) });
  };

  const closeDay = () => {
    if (busy.current || !open) return;
    if (reducedMotion()) return setOpen(null);
    busy.current = true;
    const date = open;
    gsap.to(listRef.current?.querySelector(".day-report") ?? [], {
      autoAlpha: 0,
      y: 6,
      duration: 0.15,
      ease: "power2.in",
      onComplete: () => {
        from.current = { date, offset: 0, opening: false };
        setOpen(null);
      },
    });
  };

  // Runs after the new layout is on screen: slide the row from where it was to where it is now.
  useLayoutEffect(() => {
    const move = from.current;
    const list = listRef.current;
    if (!move || !list) return;
    from.current = null;
    const row = list.querySelector<HTMLElement>(`[data-date="${move.date}"]`);
    const timeline = gsap.timeline({ onComplete: () => void (busy.current = false) });
    if (move.opening) {
      timeline
        .from(row, { y: move.offset, duration: 0.3, ease: "power3.out" })
        .from(
          list.querySelectorAll(".day-report > *"),
          { autoAlpha: 0, y: 6, duration: 0.24, stagger: 0.03, ease: "power2.out" },
          "-=0.1",
        );
    } else {
      timeline
        .from(row, { y: -rowOffset(move.date), duration: 0.3, ease: "power3.out" })
        .from(
          list.querySelectorAll(`.day-row:not([data-date="${move.date}"]), .week-summary`),
          { autoAlpha: 0, duration: 0.2, ease: "power2.out" },
          "-=0.12",
        );
    }
    return () => {
      timeline.progress(1).kill();
    };
  }, [open]);

  const openLog = days.find((d) => d.date === open);

  return (
    <div className={`days${openLog ? " is-open" : ""}`} ref={listRef}>
      {openLog ? (
        <>
          <DayRow day={openLog} open onClick={closeDay} />
          <DayReport day={openLog} onBack={closeDay} />
        </>
      ) : (
        <>
          <WeekSummary days={days} />
          {days.map((day) => (
            <DayRow key={day.date} day={day} open={false} onClick={() => openDay(day.date)} />
          ))}
        </>
      )}
    </div>
  );
}

/** Today / Yesterday / Monday, the date under it, and 💧 done/shown 🚶 done/shown. */
function DayRow({ day, open, onClick }: { day: DayLog; open: boolean; onClick: () => void }) {
  const water = tally(day, "water");
  const move = tally(day, "break");
  return (
    <button
      className={`day-row${open ? " open" : ""}`}
      data-date={day.date}
      aria-expanded={open}
      aria-label={`${dayName(day.date)}, ${shortDate(day.date)}: water ${water.done} of ${water.total}, moves ${move.done} of ${move.total}`}
      onClick={onClick}
    >
      <span className="day-label">
        <span className="day-name">{dayName(day.date)}</span>
        <span className="day-date">{shortDate(day.date)}</span>
      </span>
      <span className="chips" aria-hidden="true">
        <Chip kind="water" {...water} />
        <Chip kind="move" {...move} />
      </span>
      <span className="chevron" aria-hidden="true" />
    </button>
  );
}

/** The minutes box from the pace picker, counting cards instead of minutes. */
function Chip({ kind, done, total }: { kind: "water" | "move"; done: number; total: number }) {
  return (
    <span className={`chip ${kind}${total === 0 ? " off" : ""}`}>
      <img
        className="pace-icon"
        src={kind === "water" ? "/icons/droplet.svg" : "/icons/person-walking.svg"}
        alt=""
        draggable={false}
      />
      {total === 0 ? "0" : `${done}/${total}`}
    </span>
  );
}

/** The last 7 days (today included) in one line. */
function WeekSummary({ days }: { days: DayLog[] }) {
  const since = localDate(-6);
  const week = days.filter((d) => d.date >= since);
  const total = week.reduce((n, d) => n + d.entries.length, 0);
  if (total === 0) return null;
  const done = week.reduce((n, d) => n + d.entries.filter((e) => e.outcome === "done").length, 0);
  return (
    <p className="week-summary">
      <span className="label">Last 7 days</span>
      <span>
        <b>{done}</b> of {total} done
      </span>
    </p>
  );
}

function DayReport({ day, onBack }: { day: DayLog; onBack: () => void }) {
  return (
    <div className="day-report">
      <ol className="entries">
        {day.entries.map((entry, i) => (
          <li key={i} className={`entry ${entry.outcome}`}>
            <span className="time">{entry.time}</span>
            <span className={`kind ${entry.kind}`} aria-hidden="true">
              <img src={entry.kind === "water" ? "/icons/droplet.svg" : "/icons/person-walking.svg"} alt="" />
            </span>
            <span className="what">{entryName(entry)}</span>
            <span className="outcome">{entry.outcome === "done" ? "Done" : "Skipped"}</span>
          </li>
        ))}
      </ol>
      <button className="back" onClick={onBack}>
        Back to all days
      </button>
    </div>
  );
}

function tally(day: DayLog, kind: LogEntry["kind"]) {
  const entries = day.entries.filter((e) => e.kind === kind);
  return { done: entries.filter((e) => e.outcome === "done").length, total: entries.length };
}

function entryName(entry: LogEntry): string {
  if (entry.kind === "water") return "Water break";
  return (entry.movementId && MOVEMENTS[entry.movementId]?.name) || "Movement";
}

function localDate(offsetDays = 0): string {
  const d = new Date();
  d.setDate(d.getDate() + offsetDays);
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, "0")}-${String(d.getDate()).padStart(2, "0")}`;
}

function parse(date: string): Date {
  const [y, m, d] = date.split("-").map(Number);
  return new Date(y, m - 1, d);
}

/** "Today", "Yesterday", or the weekday: "Monday". */
function dayName(date: string): string {
  if (date === localDate()) return "Today";
  if (date === localDate(-1)) return "Yesterday";
  return parse(date).toLocaleDateString("en-GB", { weekday: "long" });
}

/** "21.10.26" */
function shortDate(date: string): string {
  const [y, m, d] = date.split("-");
  return `${d}.${m}.${y.slice(2)}`;
}
