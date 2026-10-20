import { useState } from "react";
import CharacterImage from "../content/CharacterImage";
import "../settings/onboarding.css";

/**
 * Mockup (2026-10-20): three preset pills plus a fourth, "Custom". Recommended is selected to
 * start. Only Custom opens the two sliders (water, move). Same picker in onboarding and Settings.
 * Static data, nothing saved.
 */

const WATER_OFF = 15; // the leftmost stop of the water slider means "off"
const WATER = { min: WATER_OFF, max: 120, step: 5 };
const MOVE = { min: 30, max: 120, step: 5 };

const PRESETS = [
  { id: "recommended", title: "Recommended", water: 45, move: 60 },
  { id: "active", title: "Active", water: 30, move: 45 },
];

function every(minutes: number) {
  if (minutes < 60) return `Every ${minutes} min`;
  const h = Math.floor(minutes / 60);
  const m = minutes % 60;
  return m === 0 ? `Every ${h === 1 ? "hour" : `${h} hours`}` : `Every ${h} h ${m} min`;
}

function detail(water: number, move: number) {
  const w = water === WATER_OFF ? "no water" : `water ${water}`;
  return `${w} · move ${move} min`;
}

/** Microsoft Fluent Emoji (Flat), MIT: public/icons/FLUENT-EMOJI-LICENSE.txt. Same look on Windows and Mac. */
function Icon({ name }: { name: "droplet" | "person-walking" }) {
  return <img className="chip-icon" src={`/icons/${name}.svg`} alt="" aria-hidden="true" draggable={false} />;
}

/** Two small boxes: 💧 minutes and 🚶 minutes. */
function Chips({ water, move }: { water: number; move: number }) {
  return (
    <span className="chips" aria-label={detail(water, move)}>
      <span className={`chip water${water === WATER_OFF ? " off" : ""}`}>
        <Icon name="droplet" />
        {water === WATER_OFF ? "off" : `${water}′`}
      </span>
      <span className="chip move">
        <Icon name="person-walking" />
        {`${move}′`}
      </span>
    </span>
  );
}

type Choice = { pace: string; water: number; move: number };

function PacePicker({ value, onChange, compact }: { value: Choice; onChange: (c: Choice) => void; compact?: boolean }) {
  const custom = value.pace === "custom";
  return (
    <div className={`paces${compact ? " compact" : ""}`}>
      {PRESETS.map((p) => (
        <button
          key={p.id}
          className={`pace${value.pace === p.id ? " selected" : ""}`}
          onClick={() => onChange({ pace: p.id, water: p.water, move: p.move })}
        >
          <span className="radio" />
          {compact ? (
            <>
              <span className="pace-title">{p.title}</span>
              <Chips water={p.water} move={p.move} />
            </>
          ) : (
            <span className="pace-stack">
              <span className="pace-title">
                {p.title}
                {p.id === "recommended" && <span className="tag">Best to start</span>}
              </span>
              <Chips water={p.water} move={p.move} />
            </span>
          )}
        </button>
      ))}
      <div className={`pace custom-pace${custom ? " selected open" : ""}`}>
        <button className="custom-head" onClick={() => onChange({ ...value, pace: "custom" })}>
          <span className="radio" />
          <span className="pace-title">Custom</span>
          {custom ? <Chips water={value.water} move={value.move} /> : <span className="pace-short">set your own</span>}
        </button>
        {custom && (
          <div className="sliders">
            <Slider
              label="Water"
              icon="droplet"
              value={value.water}
              {...WATER}
              readout={value.water === WATER_OFF ? "Off" : every(value.water)}
              off={value.water === WATER_OFF}
              onChange={(water) => onChange({ ...value, water })}
            />
            <Slider
              label="Move"
              icon="person-walking"
              value={value.move}
              {...MOVE}
              readout={every(value.move)}
              onChange={(move) => onChange({ ...value, move })}
            />
          </div>
        )}
      </div>
    </div>
  );
}

export default function PaceSliders() {
  const [settings, setSettings] = useState<Choice>({ pace: "recommended", water: 45, move: 60 });
  const [onboarding, setOnboarding] = useState<Choice>({ pace: "recommended", water: 45, move: 60 });

  return (
    <div className="frames">
      <figure className="frame">
        <figcaption>
          <strong>P2</strong> Onboarding, step 2. A preset tap moves on; Custom opens the sliders
        </figcaption>
        <div className="window">
          <div className="titlebar">Moovy</div>
          <main className="onboarding">
            <div className="stage">
              <p className="step">Step 2 of 2</p>
              <h1>How often?</h1>
              <PacePicker value={onboarding} onChange={setOnboarding} />
              <div className="nav">
                <button className="ghost">Back</button>
                {onboarding.pace === "custom" && <button className="primary">Next</button>}
              </div>
            </div>
          </main>
        </div>
      </figure>

      <figure className="frame">
        <figcaption>
          <strong>P1</strong> Settings: the same four, one line each (window 380 × 680)
        </figcaption>
        <div className="window tall">
          <div className="titlebar">Moovy</div>
          <main className="settings">
            <header>
              <h1>Moovy</h1>
            </header>
            <section>
              <p className="label">Your buddy</p>
              <div className="characters">
                {["thomas", "corni"].map((id) => (
                  <button key={id} className={`character${id === "corni" ? " selected" : ""}`}>
                    <span className="portrait">
                      <CharacterImage character={id} file="idle.png" />
                    </span>
                    <span className="name">{id === "thomas" ? "Thomas" : "Corni"}</span>
                  </button>
                ))}
              </div>
            </section>
            <section>
              <p className="label">Pace</p>
              <PacePicker value={settings} onChange={setSettings} compact />
            </section>
            <section className="switches">
              <label className="switch-row">
                <span>Start with Windows</span>
                <input type="checkbox" role="switch" defaultChecked />
                <span className="track" aria-hidden="true" />
              </label>
            </section>
            <footer>
              <p className="links">
                <span>v0.1.5</span>
                <button>GitHub</button>
                <button>Report a problem</button>
                <button>Clear my data</button>
              </p>
            </footer>
          </main>
        </div>
      </figure>

      <div className="slider-explain">
        <p>
          <b>Try it:</b> both start on Recommended. Click Custom to open the sliders. In onboarding, a preset moves on by
          itself; Custom shows a Next button instead.
        </p>
        <p>Onboarding now: {detail(onboarding.water, onboarding.move)} ({onboarding.pace}).</p>
        <p>Settings now: {detail(settings.water, settings.move)} ({settings.pace}).</p>
      </div>
    </div>
  );
}

function Slider(props: {
  label: string;
  icon?: "droplet" | "person-walking";
  value: number;
  min: number;
  max: number;
  step: number;
  readout: string;
  off?: boolean;
  onChange: (v: number) => void;
}) {
  const pct = ((props.value - props.min) / (props.max - props.min)) * 100;
  return (
    <div className={`slider${props.off ? " off" : ""}`}>
      <div className="slider-head">
        <span className="slider-label">
          {props.icon && <Icon name={props.icon} />}
          {props.label}
        </span>
        <span className="slider-readout">{props.readout}</span>
      </div>
      <input
        type="range"
        min={props.min}
        max={props.max}
        step={props.step}
        value={props.value}
        style={{ ["--fill" as string]: `${pct}%` }}
        onChange={(e) => props.onChange(Number(e.target.value))}
        aria-label={props.label}
      />
    </div>
  );
}
