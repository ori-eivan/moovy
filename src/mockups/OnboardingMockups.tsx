import type { ReactNode } from "react";
import CharacterImage from "../content/CharacterImage";
import FlowPrototype from "./FlowPrototype";
import PaceSliders from "./PaceSliders";
import "../settings/settings.css";
import "./mockups.css";

const BUDDIES = [
  { id: "thomas", name: "Thomas" },
  { id: "corni", name: "Corni" },
];

/** Static screens only: nothing here is wired to the app. Each frame is the real window size. */
export default function OnboardingMockups() {
  return (
    <div className="mockups">
      <header className="intro">
        <h1>Onboarding mockups</h1>
        <p>
          Three directions for the first-run flow. Every frame is the real window size (380 × 520). Each screen has an id
          (A1, B1…) to point at in your notes. Nothing here is built yet.
        </p>
      </header>

      <section className="direction">
        <h2 className="direction-title">
          <span className="badge">P</span> Water and move sliders (new, 20.10)
        </h2>
        <p className="direction-note">
          Three presets and a fourth pill, Custom. Recommended is selected to start. Only Custom opens the two
          sliders: water off or every 20 to 120 minutes, move every 30 to 120, in steps of 5.
        </p>
        <PaceSliders />
      </section>

      <section className="direction">
        <h2 className="direction-title">
          <span className="badge">★</span> Final flow, clickable
        </h2>
        <p className="direction-note">
          Decided 19.10: a tap on a choice moves on by itself, Back on screens 2 and 3, Close on the last one. Then a
          hello card on the desktop: drag it once and it saves the spot. Start with Windows is on without asking. Try
          it.
        </p>
        <FlowPrototype />
      </section>

      <h2 className="older">Earlier directions</h2>

      <Direction
        id="A"
        title="Two steps"
        note="Buddy first, then pace. Closest to today's settings window, so the least work."
      >
        <Frame id="A1" caption="Pick your buddy">
          <Step n={1} of={2} />
          <h2>Pick your wellbeing buddy</h2>
          <Buddies selected="thomas" />
          <Actions primary="Next" />
        </Frame>
        <Frame id="A2" caption="Pick your pace">
          <Step n={2} of={2} />
          <h2>How often?</h2>
          <Paces selected="recommended" />
          <Toggle label="Start with Windows" on />
          <Actions primary="Let's go" back />
        </Frame>
        <Frame id="A3" caption="Done: what happens next">
          <Done buddy="thomas" />
        </Frame>
      </Direction>

      <Direction id="B" title="One screen" note="Everything on one page. Fastest to finish, but busier.">
        <Frame id="B1" caption="All in one">
          <h2>Welcome to Moovy</h2>
          <p className="label">Your buddy</p>
          <Buddies selected="thomas" compact />
          <p className="label">Reminders</p>
          <Paces selected="recommended" compact />
          <Actions primary="Start" />
        </Frame>
      </Direction>

      <Direction
        id="C"
        title="Guided, with a preview"
        note="A hello from the buddy, then choices, then a sample card so people know what to expect in 30 minutes."
      >
        <Frame id="C1" caption="Hello">
          <div className="hello">
            <div className="hello-art">
              <CharacterImage character="thomas" file="idle.png" />
            </div>
            <h2>Hi! I'm your desk buddy.</h2>
            <p className="body">
              While you work, I'll pop up now and then to remind you to drink some water and move for 30 seconds. One
              click, then I'm gone.
            </p>
          </div>
          <Actions primary="Sounds good" />
        </Frame>
        <Frame id="C2" caption="Pick your buddy">
          <Step n={1} of={2} />
          <h2>Pick your buddy</h2>
          <Buddies selected="corni" />
          <Actions primary="Next" back />
        </Frame>
        <Frame id="C3" caption="Pace, with a timeline">
          <Step n={2} of={2} />
          <h2>Your rhythm</h2>
          <Paces selected="recommended" compact />
          <Timeline />
          <Toggle label="Quiet after 19:00" note="idea" />
          <Actions primary="Show me" back />
        </Frame>
        <Frame id="C4" caption="Sample card">
          <h2>This is what it looks like</h2>
          <p className="body">In 30 minutes of work, this card shows up at the bottom-left of your screen.</p>
          <SampleCard buddy="corni" />
          <Actions primary="Got it" />
        </Frame>
      </Direction>
    </div>
  );
}

function Direction({ id, title, note, children }: { id: string; title: string; note: string; children: ReactNode }) {
  return (
    <section className="direction">
      <h2 className="direction-title">
        <span className="badge">{id}</span> {title}
      </h2>
      <p className="direction-note">{note}</p>
      <div className="frames">{children}</div>
    </section>
  );
}

function Frame({ id, caption, children }: { id: string; caption: string; children: ReactNode }) {
  return (
    <figure className="frame">
      <figcaption>
        <strong>{id}</strong> {caption}
      </figcaption>
      <div className="window">
        <div className="titlebar">Moovy</div>
        <main className="settings onboarding">{children}</main>
      </div>
    </figure>
  );
}

function Step({ n, of }: { n: number; of: number }) {
  return (
    <p className="step">
      Step {n} of {of}
    </p>
  );
}

function Buddies({ selected, compact = false }: { selected: string; compact?: boolean }) {
  return (
    <div className={`characters${compact ? " compact" : ""}`}>
      {BUDDIES.map((b) => (
        <button key={b.id} className={`character${b.id === selected ? " selected" : ""}`}>
          <span className="portrait">
            <CharacterImage character={b.id} file="idle.png" />
          </span>
          <span className="name">{b.name}</span>
        </button>
      ))}
    </div>
  );
}

const PACES = [
  { id: "recommended", title: "Recommended", detail: "Water every 30 min · a move every 60 min" },
  { id: "relaxed", title: "Relaxed", detail: "Water every 45 min · a move every 90 min" },
  { id: "move", title: "Just moves", detail: "A move every 60 min, no water" },
];

function Paces({ selected, compact = false }: { selected: string; compact?: boolean }) {
  return (
    <div className={`paces${compact ? " compact" : ""}`}>
      {PACES.map((p) => (
        <div key={p.id} className={`pace${p.id === selected ? " selected" : ""}`}>
          <span className="radio" />
          <span>
            <span className="pace-title">
              {p.title}
              {p.id === "recommended" && <span className="tag">Best to start</span>}
            </span>
            <span className="pace-detail">{p.detail}</span>
          </span>
        </div>
      ))}
    </div>
  );
}

function Toggle({ label, on = false, note }: { label: string; on?: boolean; note?: string }) {
  return (
    <div className="toggle">
      <span>
        {label}
        {note && <span className="tag muted">{note}</span>}
      </span>
      <span className={`track${on ? " on" : ""}`} />
    </div>
  );
}

function Timeline() {
  return (
    <div className="timeline" aria-label="One hour of work">
      <div className="line" />
      <div className="tick start">
        <span className="dot" />
        <span>Start</span>
      </div>
      <div className="tick water">
        <span className="dot" />
        <span>Water · 30</span>
      </div>
      <div className="tick move">
        <span className="dot" />
        <span>Move · 60</span>
      </div>
    </div>
  );
}

function Actions({ primary, back = false }: { primary: string; back?: boolean }) {
  return (
    <div className="actions">
      {back ? <button className="ghost">Back</button> : <span />}
      <button className="primary">{primary}</button>
    </div>
  );
}

function Done({ buddy }: { buddy: string }) {
  return (
    <div className="done">
      <div className="hello-art small">
        <CharacterImage character={buddy} file="celebration.png" />
      </div>
      <h2>You're all set</h2>
      <p className="body">
        Your first water break comes after 30 minutes of work. Moovy lives in the tray, next to the clock.
      </p>
      <p className="hint">You can change all of this later: tray → Settings.</p>
      <div className="actions">
        <span />
        <button className="primary">Close</button>
      </div>
    </div>
  );
}

function SampleCard({ buddy }: { buddy: string }) {
  return (
    <div className="sample-card">
      <div className="sample-art">
        <CharacterImage character={buddy} file="water.png" />
      </div>
      <div>
        <p className="sample-title">Water break?</p>
        <p className="sample-line">A few sips is plenty.</p>
        <div className="sample-buttons">
          <span className="pill dark">Done</span>
          <span className="pill">Snooze</span>
        </div>
      </div>
    </div>
  );
}
