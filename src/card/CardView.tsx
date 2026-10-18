import { forwardRef, type MouseEvent } from "react";
import CharacterImage from "../content/CharacterImage";
import { DEFAULT_CHARACTER } from "../content/characters";
import { SWITCH_SIDES_MS, type CardActions, type View } from "./useCardFlow";
import "./card.css";

interface Props {
  view: View;
  now: number;
  actions: CardActions;
  /** Which character pack the images come from. */
  character?: string;
  onMouseDown?: (event: MouseEvent) => void;
}

/** How the card looks in each state. No behaviour here: that lives in useCardFlow. */
const CardView = forwardRef<HTMLDivElement, Props>(function CardView(
  { view, now, actions, character = DEFAULT_CHARACTER, onMouseDown },
  ref,
) {
  return (
    <div
      className={`card view-${view.name}${view.name === "movement" && view.howTo ? " howto-open" : ""}`}
      ref={ref}
      onMouseDown={onMouseDown}
      role="dialog"
      aria-label="Moovy reminder"
    >
      <Mascot character={character} image={imageFor(view)} />
      <div className="body">{renderBody(view, now, actions)}</div>
      {view.name === "movement" && (
        // Slides in over the mascot and text, from right to left. The buttons below it never move.
        <div className="howto-pane" aria-hidden={!view.howTo}>
          <p>{view.movement.howTo}</p>
          <p className="muted">{view.movement.alternative}</p>
          <p className="dose">{view.movement.dose}</p>
        </div>
      )}
      {showsClose(view) && (
        <button className="close" aria-label="Skip" onClick={actions.skip}>
          ×
        </button>
      )}
    </div>
  );
});

export default CardView;

function Mascot({ character, image }: { character: string; image: string }) {
  return (
    <div className="mascot" aria-hidden="true">
      <CharacterImage character={character} file={image} />
    </div>
  );
}

function imageFor(view: View): string {
  switch (view.name) {
    case "breakIntro":
      return "break.png";
    case "movement":
    case "running":
      return view.movement.image;
    case "celebration":
      return "celebration.png";
    case "water":
    case "waterDone":
      return "water.png";
    case "waiting":
      return "waiting.png";
  }
}

function showsClose(view: View): boolean {
  return view.name === "breakIntro" || view.name === "movement" || view.name === "running" || view.name === "water";
}

function renderBody(view: View, now: number, actions: CardActions) {
  switch (view.name) {
    case "breakIntro":
      return (
        <>
          <p className="small">It's time to</p>
          <p className="big accent">{view.movement.category === "stretch" ? "stretch!" : "move!"}</p>
          <p className="line">Quick one while you're here.</p>
        </>
      );

    case "movement": {
      const m = view.movement;
      return (
        <>
          <div className="summary">
            <p className="title">{m.name}</p>
            <p className="line">{m.cardLine}</p>
            <p className="dose">{m.dose}</p>
          </div>
          <div className="actions">
            {m.mode === "reps" ? (
              <button className="primary" onClick={actions.repsDone}>
                Done
              </button>
            ) : (
              <button className="primary" onClick={() => actions.start(m)}>
                Start
              </button>
            )}
            <button onClick={actions.snooze}>Snooze</button>
            <button className="link" onClick={actions.toggleHowTo}>
              {view.howTo ? "Back" : "How to"}
            </button>
          </div>
        </>
      );
    }

    case "running": {
      const m = view.movement;
      const total = view.durationMs;
      const left = Math.max(0, view.endsAt - now);
      const elapsed = total - left;
      const half = total / 2;
      const switching = m.twoSided && elapsed >= half && elapsed < half + Math.min(SWITCH_SIDES_MS, half);
      // Shown in real seconds even when the gallery runs the timer faster.
      const secondsLeft = Math.ceil((left / total) * m.seconds);
      return (
        <>
          <p className="title">{m.name}</p>
          <p className={switching ? "switch" : "line"}>{switching ? "Switch sides" : m.cardLine}</p>
          <p className="countdown" aria-live="polite">
            {Math.floor(secondsLeft / 60)}:{String(secondsLeft % 60).padStart(2, "0")}
          </p>
          <div className="progress" aria-hidden="true">
            <div style={{ width: `${(elapsed / total) * 100}%` }} />
          </div>
        </>
      );
    }

    case "celebration":
      return (
        <>
          <p className="big">Nice one!</p>
          <p className="line">Back to it.</p>
        </>
      );

    case "water":
      return (
        <>
          <p className="title">Water break?</p>
          <p className="line">A few sips is plenty.</p>
          <div className="actions">
            <button className="primary" onClick={actions.waterDone}>
              Done
            </button>
            <button onClick={actions.snooze}>Snooze</button>
          </div>
        </>
      );

    case "waterDone":
      return <p className="big">Refreshed!</p>;

    case "waiting":
      return <p className="big">See you in 10</p>;
  }
}
