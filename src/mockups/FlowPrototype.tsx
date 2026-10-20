import { useLayoutEffect, useRef, useState } from "react";
import { gsap } from "gsap";
import type { PointerEvent as ReactPointerEvent } from "react";
import CharacterImage from "../content/CharacterImage";

/**
 * Clickable prototype of the agreed first-run flow (decided 2026-10-19):
 * a tap on a choice moves on by itself, Back on screens 2 and 3, Close on the last one,
 * then a hello card on the "desktop" with a bubble: drag it once and it says it saved the spot.
 */

const BUDDIES = [
  { id: "thomas", name: "Thomas" },
  { id: "corni", name: "Corni" },
];

const PACES = [
  { id: "recommended", title: "Recommended", detail: "Water every 30 min · a move every 60 min" },
  { id: "relaxed", title: "Relaxed", detail: "Water every 45 min · a move every 90 min" },
  { id: "move", title: "Just moves", detail: "A move every 60 min, no water" },
];

/** Long enough to see the choice land before the screen starts to frost over. */
const SELECT_HOLD = 0.18;

const reducedMotion = () => window.matchMedia("(prefers-reduced-motion: reduce)").matches;

type Stage = 1 | 2 | 3 | "demo" | "finished";

export default function FlowPrototype() {
  const [stage, setStage] = useState<Stage>(1);
  const [buddy, setBuddy] = useState("thomas");
  const [pace, setPace] = useState("recommended");
  const [leaving, setLeaving] = useState(false);

  const content = useRef<HTMLDivElement>(null);
  const glass = useRef<HTMLDivElement>(null);
  const direction = useRef<1 | -1>(1);

  /**
   * Frosted-glass transition: the screen blurs and fades under a sheet of frosted glass, the
   * next screen is swapped in behind it, then comes into focus as the glass clears.
   */
  const go = (next: Stage, dir: 1 | -1 = 1) => {
    if (leaving) return;
    direction.current = dir;
    if (reducedMotion() || !content.current) {
      setStage(next);
      return;
    }
    setLeaving(true);
    gsap
      .timeline({ onComplete: () => setStage(next) })
      .to({}, { duration: dir === 1 ? SELECT_HOLD : 0 })
      .to(content.current, { autoAlpha: 0, filter: "blur(10px)", scale: 0.97, y: -10 * dir, duration: 0.3, ease: "power2.in" })
      .to(glass.current, { autoAlpha: 1, duration: 0.24, ease: "power1.out" }, "<0.06");
  };

  const choose = (apply: () => void, next: Stage) => {
    if (leaving) return;
    apply();
    go(next);
  };

  // The new screen comes into focus as the glass clears.
  useLayoutEffect(() => {
    setLeaving(false);
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
  }, [stage]);

  const firstCard =
    pace === "move" ? "first move comes after 60 minutes" : `first water break comes after ${pace === "relaxed" ? 45 : 30} minutes`;

  return (
    <div className="prototype">
      <div className="desktop">
        {(stage === 1 || stage === 2 || stage === 3) && (
          <div className="window centered">
            <div className="titlebar">Moovy</div>
            <main className="settings onboarding glassy">
              <div className="glass" ref={glass} aria-hidden="true" />
              <div className="stage" ref={content}>
              {stage === 1 && (
                <>
                  <p className="step">Step 1 of 2</p>
                  <h2>Pick your wellbeing buddy</h2>
                  <div className="characters" role="radiogroup">
                    {BUDDIES.map((b) => (
                      <button
                        key={b.id}
                        role="radio"
                        aria-checked={buddy === b.id}
                        className={`character${buddy === b.id ? " selected" : ""}`}
                        onClick={() => choose(() => setBuddy(b.id), 2)}
                      >
                        <span className="portrait">
                          <CharacterImage character={b.id} file="idle.png" />
                        </span>
                        <span className="name">{b.name}</span>
                      </button>
                    ))}
                  </div>
                  <p className="hint">Tap one to continue.</p>
                </>
              )}
              {stage === 2 && (
                <>
                  <p className="step">Step 2 of 2</p>
                  <h2>How often?</h2>
                  <div className="paces" role="radiogroup">
                    {PACES.map((p) => (
                      <button
                        key={p.id}
                        role="radio"
                        aria-checked={pace === p.id}
                        className={`pace${pace === p.id ? " selected" : ""}`}
                        onClick={() => choose(() => setPace(p.id), 3)}
                      >
                        <span className="radio" />
                        <span>
                          <span className="pace-title">
                            {p.title}
                            {p.id === "recommended" && <span className="tag">Best to start</span>}
                          </span>
                          <span className="pace-detail">{p.detail}</span>
                        </span>
                      </button>
                    ))}
                  </div>
                  <div className="actions">
                    <button className="ghost" onClick={() => go(1, -1)}>
                      Back
                    </button>
                    <span />
                  </div>
                </>
              )}
              {stage === 3 && (
                <div className="done">
                  <div className="hello-art small">
                    <CharacterImage character={buddy} file="celebration.png" />
                  </div>
                  <h2>You're all set</h2>
                  <p className="body">
                    Your {firstCard} of work. Moovy lives in the tray, next to the clock.
                  </p>
                  <p className="hint">You can change all of this later: tray → Settings.</p>
                  <div className="actions">
                    <button className="ghost" onClick={() => go(2, -1)}>
                      Back
                    </button>
                    <button className="primary" onClick={() => go("demo")}>
                      Close
                    </button>
                  </div>
                </div>
              )}
              </div>
            </main>
          </div>
        )}

        {stage === "demo" && <DemoCard buddy={buddy} onDone={() => setStage("finished")} />}

        {stage === "finished" && (
          <div className="finished">
            <p>Onboarding finished. From here Moovy waits quietly in the tray.</p>
            <button className="restart" onClick={() => setStage(1)}>
              Run it again
            </button>
          </div>
        )}
        <div className="taskbar" />
      </div>
    </div>
  );
}

/** The hello card with its bubble. The first drag saves the spot, then it closes by itself. */
function DemoCard({ buddy, onDone }: { buddy: string; onDone: () => void }) {
  const [pos, setPos] = useState({ x: 16, y: 0 });
  const [saved, setSaved] = useState(false);
  const drag = useRef<{ dx: number; dy: number } | null>(null);
  const cardRef = useRef<HTMLDivElement>(null);

  // Comes into focus the same way the onboarding screens do, then the bubble pops in.
  useLayoutEffect(() => {
    if (!cardRef.current || reducedMotion()) return;
    const tl = gsap
      .timeline()
      .fromTo(
        cardRef.current.querySelector(".hello-card"),
        { autoAlpha: 0, filter: "blur(10px)", y: 16 },
        { autoAlpha: 1, filter: "blur(0px)", y: 0, duration: 0.45, ease: "power3.out", clearProps: "filter,transform" },
      )
      .fromTo(
        cardRef.current.querySelector(".bubble"),
        { autoAlpha: 0, y: 6, scale: 0.94 },
        { autoAlpha: 1, y: 0, scale: 1, duration: 0.35, ease: "back.out(1.6)" },
        "-=0.1",
      );
    return () => {
      tl.kill();
    };
  }, []);

  const onDown = (e: ReactPointerEvent<HTMLDivElement>) => {
    if ((e.target as HTMLElement).closest("button") || saved) return;
    drag.current = { dx: e.clientX - pos.x, dy: e.clientY - pos.y };
    e.currentTarget.setPointerCapture(e.pointerId);
  };
  const onMove = (e: ReactPointerEvent<HTMLDivElement>) => {
    if (!drag.current) return;
    setPos({ x: e.clientX - drag.current.dx, y: e.clientY - drag.current.dy });
  };
  const onUp = () => {
    if (!drag.current) return;
    drag.current = null;
    setSaved(true);
    window.setTimeout(onDone, 2000);
  };

  return (
    <div
      ref={cardRef}
      className={`demo${saved ? " saved" : ""}`}
      style={{ left: pos.x, bottom: 56 - pos.y }}
      onPointerDown={onDown}
      onPointerMove={onMove}
      onPointerUp={onUp}
    >
      <div className="bubble" role="status">
        {saved ? "Saved! I'll show up right here." : "Drag me anywhere. I'll remember the spot."}
      </div>
      <div className="sample-card hello-card">
        <div className="sample-art">
          <CharacterImage character={buddy} file="idle.png" />
        </div>
        <div>
          <p className="sample-title">Hi! This is where I'll pop up.</p>
          <p className="sample-line">Every now and then, while you work.</p>
        </div>
        <button className="close" aria-label="Close" onClick={onDone}>
          ×
        </button>
      </div>
    </div>
  );
}
