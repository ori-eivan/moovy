// The characters offered in settings. Each one is a character pack in public/characters/<id>/
// (docs/ARCHITECTURE.md §4). A pack with missing images falls back to break.png, then to the
// file name, so an unfinished pack never breaks a card.

import { useEffect, useState } from "react";

export interface Character {
  id: string;
  name: string;
}

export const CHARACTERS: Character[] = [
  { id: "thomas", name: "Thomas" },
  { id: "corni", name: "Corni" },
  { id: "pilot-1", name: "Character 1" },
  { id: "pilot-3", name: "Character 3" },
];

/** Must match DEFAULT_CHARACTER in settings.rs. */
export const DEFAULT_CHARACTER = "thomas";

/**
 * Only the characters whose art is actually on this machine. Packs that are not in the
 * repository (local test art) simply don't appear for anyone who doesn't have them.
 */
export function useAvailableCharacters(): Character[] {
  const [available, setAvailable] = useState<Character[]>(() => CHARACTERS.filter((c) => c.id === DEFAULT_CHARACTER));
  useEffect(() => {
    let cancelled = false;
    const checks = CHARACTERS.map(
      (c) =>
        new Promise<Character | null>((resolve) => {
          const img = new Image();
          img.onload = () => resolve(c);
          img.onerror = () => resolve(null);
          img.src = `/characters/${c.id}/idle.png`;
        }),
    );
    Promise.all(checks).then((found) => {
      if (!cancelled) setAvailable(found.filter((c): c is Character => c !== null));
    });
    return () => {
      cancelled = true;
    };
  }, []);
  return available;
}

export function characterName(id: string): string {
  return CHARACTERS.find((c) => c.id === id)?.name ?? id;
}
