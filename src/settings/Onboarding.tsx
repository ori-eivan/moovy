import { useLayoutEffect, useRef, useState } from "react";
import { gsap } from "gsap";
import CharacterImage from "../content/CharacterImage";
import { DEFAULT_CHARACTER, useAvailableCharacters } from "../content/characters";
import { DEFAULT_PACE, firstCardLine, type Pace } from "../content/paces";
import PacePicker from "./PacePicker";
import "./onboarding.css";

interface Props {
  /** The last screen was closed. */
  onFinish: (choice: { character: string; pace: Pace }) => void;
}

type Step = 1 | 2 | 3;

/** Long enough to see the choice land before the screen starts to frost over. */
const SELECT_HOLD = 0.18;

/** The tray is the menu bar on Mac. */
const IS_MAC = navigator.userAgent.includes("Mac");

const reducedMotion = () => window.matchMedia("(prefers-reduced-motion: reduce)").matches;

/**
 * First run, inside the settings window (decided 2026-10-19): pick a buddy, pick a pace, done.
 * A tap on a choice moves on by itself; Back on screens 2 and 3; Close on the last one.
 * Screens change behind a sheet of frosted glass.
 */
export default function Onboarding({ onFinish }: Props) {
  const characters = useAvailableCharacters();
  const [step, setStep] = useState<Step>(1);
  const [character, setCharacter] = useState(DEFAULT_CHARACTER);
  const [pace, setPace] = useState<Pace>(DEFAULT_PACE);
  const [busy, setBusy] = useState(false);
  const content = useRef<HTMLDivElement>(null);
  const glass = useRef<HTMLDivElement>(null);
  const direction = useRef<1 | -1>(1);

  /** Frost the current screen over, then run `then` (switch screen, or finish). */
  const leave = (then: () => void, dir: 1 | -1 = 1) => {
    if (busy) return;
    direction.current = dir;
    if (reducedMotion() || !content.current) {
      then();
      return;
    }
    setBusy(true);
    gsap
      .timeline({ onComplete: then })
      .to({}, { duration: dir === 1 ? SELECT_HOLD : 0 })
      .to(content.current, { autoAlpha: 0, filter: "blur(10px)", scale: 0.97, y: -10 * dir, duration: 0.3, ease: "power2.in" })
      .to(glass.current, { autoAlpha: 1, duration: 0.24, ease: "power1.out" }, "<0.06");
  };

  const go = (next: Step, dir: 1 | -1 = 1) => leave(() => setStep(next), dir);

  // The new screen comes into focus as the glass clears.
  useLayoutEffect(() => {
    setBusy(false);
    if (!content.current || reducedMotion()) {
      gsap.set(glass.current, { autoAlpha: 0 });
      return;
    }
    const dir = direction.current;
    const tl = gsap
      .timeline()
      .fromTo(
        content.current,
        { autoAlpha: 0, filter: "blur(12px)", scale: 1.02, y: 14 * dir },
        { autoAlpha: 1, filter: "blur(0px)", scale: 1, y: 0, duration: 0.45, ease: "power3.out", clearProps: "filter,transform" },
      )
      .to(glass.current, { autoAlpha: 0, duration: 0.4, ease: "power2.out" }, "<0.05");
    return () => {
      tl.kill();
    };
  }, [step]);

  return (
    <main className="onboarding">
      <div className="glass" ref={glass} aria-hidden="true" />
      <div className="stage" ref={content}>
        {step === 1 && (
          <>
            <p className="step">Step 1 of 2</p>
            <h1>Pick your wellbeing buddy</h1>
            <div className="buddies" role="radiogroup" aria-label="Your buddy">
              {characters.map((c) => (
                <button
                  key={c.id}
                  role="radio"
                  aria-checked={character === c.id}
                  className={`buddy${character === c.id ? " selected" : ""}`}
                  onClick={() => {
                    if (busy) return;
                    setCharacter(c.id);
                    go(2);
                  }}
                >
                  <span className="portrait">
                    <CharacterImage character={c.id} file="idle.png" />
                  </span>
                  <span className="name">{c.name}</span>
                </button>
              ))}
            </div>
            <p className="hint">Tap one to continue. You can switch later.</p>
          </>
        )}

        {step === 2 && (
          <>
            <p className="step">Step 2 of 2</p>
            <h1>How often?</h1>
            <PacePicker
              value={pace}
              onChange={(next, kind) => {
                if (busy) return;
                setPace(next);
                // A preset moves on by itself; Custom opens the sliders and waits for Next.
                if (kind === "preset") go(3);
              }}
            />
            <div className="nav">
              <button className="ghost" onClick={() => go(1, -1)}>
                Back
              </button>
              {pace.id === "custom" && (
                <button className="primary" onClick={() => go(3)}>
                  Next
                </button>
              )}
            </div>
          </>
        )}

        {step === 3 && (
          <>
            <div className="art">
              <CharacterImage character={character} file="celebration.png" />
            </div>
            <h1>You're all set</h1>
            <p className="lead">
              Your {firstCardLine(pace)} of work. Moovy lives {IS_MAC ? "in the menu bar, at the top right" : "in the tray, next to the clock"}.
            </p>
            <p className="hint">You can change all of this later: {IS_MAC ? "menu bar" : "tray"} → Settings.</p>
            <div className="nav">
              <button className="ghost" onClick={() => go(2, -1)}>
                Back
              </button>
              <button className="primary" onClick={() => leave(() => onFinish({ character, pace }))}>
                Close
              </button>
            </div>
          </>
        )}
      </div>
    </main>
  );
}
