import { useEffect, useState } from "react";

interface Props {
  character: string;
  /** File name inside the character pack, e.g. "wall-angels.png". */
  file: string;
}

/**
 * One image from a character pack in public/characters/<id>/.
 * A missing pose falls back to break.png (ARCHITECTURE.md §4), and a pack with no art at all
 * shows the file name, so a missing image never breaks a card.
 */
export default function CharacterImage({ character, file }: Props) {
  const candidates = file === "break.png" ? [file] : [file, "break.png"];
  const [attempt, setAttempt] = useState(0);

  useEffect(() => setAttempt(0), [character, file]);

  if (attempt >= candidates.length) {
    return <span className="placeholder">{`${character}/${file}`}</span>;
  }
  return (
    <img
      key={`${character}/${candidates[attempt]}`}
      src={`/characters/${character}/${candidates[attempt]}`}
      alt=""
      draggable={false}
      onError={() => setAttempt((a) => a + 1)}
    />
  );
}
