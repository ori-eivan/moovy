import { useEffect, useRef, useState } from "react";
import CardView from "../card/CardView";
import SettingsView, { type SettingsActions, type SettingsState } from "../settings/SettingsView";
import { useCardFlow, type CardActions, type Payload, type View } from "../card/useCardFlow";
import { MOVEMENTS } from "../content/movements";
import Typography from "./Typography";
import "./gallery.css";

// Same order as the app's rotation in card.rs.
const ROTATION = ["wall-angels", "neck-flexion", "spine-twist", "upper-trap", "hip-flexor", "doorway-chest"];
const GALLERY_SNOOZE_MS = 5000;

const noop: CardActions = {
  start: () => {},
  repsDone: () => {},
  snooze: () => {},
  waterDone: () => {},
  toggleHowTo: () => {},
  skip: () => {},
};

export default function Gallery() {
  return (
    <main className="gallery">
      <header>
        <h1>Moovy screens</h1>
        <p>Every card screen at real size. Part 1 runs the real card flow: click it like in the app. Part 2 shows every screen side by side, in the order a click leads to it.</p>
      </header>
      <TryIt />
      <AllScreens />
      <Typography />
    </main>
  );
}

/** Part 1: the real flow, clickable. */
function TryIt() {
  const [visible, setVisible] = useState(false);
  const [fast, setFast] = useState(true);
  const [log, setLog] = useState<string[]>([]);
  const [index, setIndex] = useState(0);
  const snoozeTimer = useRef<number | undefined>(undefined);
  const lastPayload = useRef<Payload | undefined>(undefined);

  const note = (line: string) => setLog((l) => [`${new Date().toLocaleTimeString()}  ${line}`, ...l].slice(0, 6));

  const { view, now, show, actions } = useCardFlow(
    {
      done: (kind) => {
        setVisible(false);
        note(kind === "break" ? "Card closed: break completed" : "Card closed: water done");
      },
      skip: () => {
        setVisible(false);
        note("Card closed with ×: skipped, nothing counted");
      },
      snooze: () => {
        setVisible(false);
        note(`Snoozed. In the app it returns after 10 minutes. Here: ${GALLERY_SNOOZE_MS / 1000} seconds`);
        snoozeTimer.current = window.setTimeout(() => {
          if (lastPayload.current) open(lastPayload.current);
        }, GALLERY_SNOOZE_MS);
      },
    },
    { timeScale: fast ? 0.1 : 1 },
  );

  const open = (payload: Payload) => {
    window.clearTimeout(snoozeTimer.current);
    lastPayload.current = payload;
    show(payload);
    setVisible(true);
  };

  useEffect(() => () => window.clearTimeout(snoozeTimer.current), []);

  const nextBreak = () => {
    const movementId = ROTATION[index % ROTATION.length];
    setIndex((i) => i + 1);
    open({ kind: "break", movementId });
    note(`Break card: ${MOVEMENTS[movementId].name}`);
  };

  return (
    <section>
      <h2>
        <span className="step">Part 1</span> Try it
      </h2>
      <div className="controls">
        <button className="primary" onClick={nextBreak}>
          Show break card (next movement)
        </button>
        <button
          onClick={() => {
            open({ kind: "water" });
            note("Water card");
          }}
        >
          Show water card
        </button>
        <label>
          <input type="checkbox" checked={fast} onChange={(e) => setFast(e.target.checked)} /> Fast timer (10×)
        </label>
      </div>
      <div className="desk desk-live">
        {visible ? <CardView view={view} now={now} actions={actions} /> : <p className="empty">No card on screen. Use the buttons above.</p>}
      </div>
      <ul className="log">{log.map((line) => <li key={line}>{line}</li>)}</ul>
    </section>
  );
}

/** Part 2: every screen, frozen, numbered in click order. */
function AllScreens() {
  const now = Date.now();
  const wall = MOVEMENTS["wall-angels"];
  const hip = MOVEMENTS["hip-flexor"];
  const neck = MOVEMENTS["neck-flexion"];

  const screens: { title: string; how: string; view: View }[] = [
    { title: "Break intro", how: "Appears on its own every hour. Lasts 3 seconds.", view: { name: "breakIntro", movement: wall } },
    {
      title: "Movement, reps",
      how: "Follows the intro. Counted in reps, so no timer: “Done” closes the card.",
      view: { name: "movement", movement: wall, howTo: false },
    },
    { title: "How to, open", how: "Click “How to” on screen 2 or 4.", view: { name: "movement", movement: wall, howTo: true } },
    {
      title: "Movement, timed",
      how: "Follows the intro. Held for 30 seconds: “Start” runs the timer.",
      view: { name: "movement", movement: hip, howTo: false },
    },
    {
      title: "Running",
      how: "Click “Start” on screen 4.",
      view: { name: "running", movement: neck, endsAt: now + 21_000, durationMs: 30_000 },
    },
    {
      title: "Switch sides",
      how: "Halfway through a two-sided movement, for 3 seconds.",
      view: { name: "running", movement: hip, endsAt: now + 14_000, durationMs: 30_000 },
    },
    { title: "Celebration", how: "When the timer ends. Closes after 4 seconds.", view: { name: "celebration" } },
    { title: "Water", how: "Appears on its own, at the half hour.", view: { name: "water" } },
    { title: "Water done", how: "Click “Done” on screen 8. Closes after about a second.", view: { name: "waterDone" } },
    { title: "Waiting", how: "Click “Snooze” on screen 2, 4 or 8. Closes after 2 seconds, the card returns in 10 minutes.", view: { name: "waiting" } },
  ];

  return (
    <section>
      <h2>
        <span className="step">Part 2</span> Every screen, in click order
      </h2>
      <div className="grid">
        {screens.map((s, i) => (
          <figure key={s.title}>
            <figcaption>
              <span className="num">{i + 1}</span>
              <span>
                <b>{s.title}</b>
                <small>{s.how}</small>
              </span>
            </figcaption>
            <div className="desk">
              <CardView view={s.view} now={now} actions={noop} />
            </div>
          </figure>
        ))}
        <SettingsScreens />
      </div>
    </section>
  );
}

/** Screens 10 and 11: the one settings window. Clickable here too, nothing is saved. */
function SettingsScreens() {
  const [state, setState] = useState<SettingsState>({
    waterEnabled: true,
    autostart: true,
    character: "pilot-1",
    version: "0.1.0",
  });
  const actions: SettingsActions = {
    setCharacter: (id) => setState((s) => ({ ...s, character: id })),
    setWater: (waterEnabled) => setState((s) => ({ ...s, waterEnabled })),
    setAutostart: (autostart) => setState((s) => ({ ...s, autostart })),
    clearData: () => {},
    openGitHub: () => {},
    reportProblem: () => {},
  };
  const screens = [
    { title: "Settings", how: "Opens on first launch, and from “Settings” in the tray.", confirming: false },
    { title: "Clear my data", how: "Click “Clear my data” on screen 11.", confirming: true },
  ];
  return (
    <>
      {screens.map((s, i) => (
        <figure key={s.title}>
          <figcaption>
            <span className="num">{11 + i}</span>
            <span>
              <b>{s.title}</b>
              <small>{s.how}</small>
            </span>
          </figcaption>
          <div className="window">
            <div className="titlebar">Moovy</div>
            <SettingsView state={state} actions={actions} confirmingClear={s.confirming} />
          </div>
        </figure>
      ))}
    </>
  );
}
