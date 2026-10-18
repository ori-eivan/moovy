// The characters offered in settings. Each one is a character pack (docs/ARCHITECTURE.md §4).
// These are placeholders until the real art arrives: the tiles show the file each image comes from.

export interface Character {
  id: string;
  name: string;
}

export const CHARACTERS: Character[] = [
  { id: "pilot-1", name: "Character 1" },
  { id: "pilot-2", name: "Character 2" },
  { id: "pilot-3", name: "Character 3" },
];

/** Must match DEFAULT_CHARACTER in settings.rs. */
export const DEFAULT_CHARACTER = "pilot-1";

export function characterName(id: string): string {
  return CHARACTERS.find((c) => c.id === id)?.name ?? id;
}
