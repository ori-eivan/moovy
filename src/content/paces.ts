// How often cards come: a preset, or custom minutes (decided 2026-10-20).
// Ids, minutes and ranges must match src-tauri/src/pace.rs.

export type PaceId = "recommended" | "active" | "custom";

export interface Pace {
  id: PaceId;
  /** null: no water. */
  waterMinutes: number | null;
  moveMinutes: number;
}

export interface Preset extends Pace {
  title: string;
}

export const PRESETS: Preset[] = [
  { id: "recommended", title: "Recommended", waterMinutes: 45, moveMinutes: 60 },
  { id: "active", title: "Active", waterMinutes: 30, moveMinutes: 45 },
];

export const DEFAULT_PACE: Pace = { id: "recommended", waterMinutes: 45, moveMinutes: 60 };

/** Water: off, or 20 to 120 minutes. Move: 30 to 120. Steps of 5. */
export const WATER_RANGE = { min: 20, max: 120 };
export const MOVE_RANGE = { min: 30, max: 120 };
export const STEP = 5;

/** "Every 45 min", "Every hour", "Every 1 h 30 min". */
export function every(minutes: number): string {
  if (minutes < 60) return `Every ${minutes} min`;
  const h = Math.floor(minutes / 60);
  const m = minutes % 60;
  if (m === 0) return h === 1 ? "Every hour" : `Every ${h} hours`;
  return `Every ${h} h ${m} min`;
}

/** Spoken summary, for screen readers and tooltips: "water every 45 min, a move every 60 min". */
export function describe(pace: Pace): string {
  const water = pace.waterMinutes === null ? "no water" : `water every ${pace.waterMinutes} min`;
  return `${water}, a move every ${pace.moveMinutes} min`;
}

/** "first water break comes after 45 minutes", to finish "Your … of work." */
export function firstCardLine(pace: Pace): string {
  const water = pace.waterMinutes;
  if (water !== null && water < pace.moveMinutes) return `first water break comes after ${water} minutes`;
  return `first move comes after ${pace.moveMinutes} minutes`;
}
