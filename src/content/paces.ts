// The pace choices. Ids and timings must match `Pace` in scheduler.rs.

export type PaceId = "recommended" | "relaxed" | "moves-only";

export interface PaceOption {
  id: PaceId;
  title: string;
  detail: string;
  /** One-line version for the settings window. */
  short: string;
}

export const DEFAULT_PACE: PaceId = "recommended";

export const PACES: PaceOption[] = [
  { id: "recommended", title: "Recommended", detail: "Water every 30 min · a move every 60 min", short: "water 30 · move 60" },
  { id: "relaxed", title: "Relaxed", detail: "Water every 45 min · a move every 90 min", short: "water 45 · move 90" },
  { id: "moves-only", title: "Just moves", detail: "A move every 60 min, no water", short: "move 60, no water" },
];

/** "first water break comes after 30 minutes", to finish "Your … of work." */
export function firstCardLine(pace: PaceId): string {
  if (pace === "moves-only") return "first move comes after 60 minutes";
  return `first water break comes after ${pace === "relaxed" ? 45 : 30} minutes`;
}
