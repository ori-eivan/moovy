import { forwardRef, useLayoutEffect, useRef, type MouseEvent } from "react";
import { gsap } from "gsap";
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
        <button
          className="close"
          aria-label={view.name === "hello" ? "Close" : "Skip"}
          onClick={view.name === "hello" ? actions.helloDone : actions.skip}
        >
          ×
        </button>
      )}
      {view.name === "hello" && <HelloBubble saved={view.saved} />}
    </div>
  );
});

export default CardView;

/** The speech bubble above the hello card. Pops in once, then only its text changes. */
function HelloBubble({ saved }: { saved: boolean }) {
  const ref = useRef<HTMLDivElement>(null);
  useLayoutEffect(() => {
    if (!ref.current || window.matchMedia("(prefers-reduced-motion: reduce)").matches) return;
    const tween = gsap.fromTo(
      ref.current,
      { autoAlpha: 0, y: 6, scale: 0.94 },
      { autoAlpha: 1, y: 0, scale: 1, duration: 0.35, delay: 0.25, ease: "back.out(1.6)" },
    );
    return () => {
      tween.kill();
    };
  }, []);
  return (
    <div className="bubble" ref={ref} role="status">
      {saved ? "Saved! I'll show up right here." : "Drag me anywhere. I'll remember the spot."}
    </div>
  );
}

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
    case "hello":
    case "updatesNotice":
    case "updateReady":
      return "idle.png";
  }
}

function showsClose(view: View): boolean {
  return (
    view.name === "breakIntro" ||
    view.name === "movement" ||
    view.name === "running" ||
    view.name === "water" ||
    view.name === "hello"
  );
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

    case "hello":
      return (
        <>
          <p className="title">Hi! This is where I'll pop up.</p>
          <p className="line">Every now and then, while you work.</p>
        </>
      );

    case "updatesNotice":
      return (
        <>
          <p className="title">Shall I keep myself up to date?</p>
          <p className="line">Once a day I ask GitHub for the newest version number. Nothing about you is sent.</p>
          <div className="actions">
            <button className="primary" onClick={() => actions.updatesAnswer(true)}>
              Keep on
            </button>
            <button onClick={() => actions.updatesAnswer(false)}>Turn off</button>
          </div>
        </>
      );

    case "updateReady":
      return (
        <>
          <p className="title">A new Moovy is ready</p>
          <p className="line">Version {view.version}. Install it now? It takes a few seconds.</p>
          <div className="actions">
            <button className="primary" onClick={actions.updateInstall}>
              Install
            </button>
            <button onClick={actions.updateLater}>Later</button>
          </div>
        </>
      );
  }
}
