import { useState } from "react";
import CharacterImage from "../content/CharacterImage";
import { useAvailableCharacters } from "../content/characters";
import type { Pace } from "../content/paces";
import PacePicker from "./PacePicker";
import "./onboarding.css";
import "./settings.css";

export interface SettingsState {
  pace: Pace;
  /** False until the first-run onboarding is finished. */
  onboarded: boolean;
  autostart: boolean;
  /** Check GitHub for a new version once a day. */
  updates: boolean;
  character: string;
  version: string;
}

export interface SettingsActions {
  setCharacter: (id: string) => void;
  setPace: (pace: Pace) => void;
  setAutostart: (enabled: boolean) => void;
  setUpdates: (enabled: boolean) => void;
  clearData: () => void;
  openGitHub: () => void;
  reportProblem: () => void;
}

interface Props {
  state: SettingsState;
  actions: SettingsActions;
  /** Gallery only: start with the "Clear my data" confirmation open. */
  confirmingClear?: boolean;
}

/** Each system calls it by its own name. */
const START_AT_LOGIN = navigator.userAgent.includes("Mac") ? "Start at login" : "Start with Windows";

/** The one Moovy window: pick a character, pick a pace, two switches, and a footer. Nothing else. */
export default function SettingsView({ state, actions, confirmingClear = false }: Props) {
  const [confirming, setConfirming] = useState(confirmingClear);
  const [cleared, setCleared] = useState(false);
  const characters = useAvailableCharacters();

  return (
    <main className="settings">
      <header>
        <h1>Moovy</h1>
      </header>

      <section aria-labelledby="character-label">
        <p className="label" id="character-label">
          Your buddy
        </p>
        <div className="characters" role="radiogroup" aria-labelledby="character-label">
          {characters.map((c) => (
            <button
              key={c.id}
              role="radio"
              aria-checked={state.character === c.id}
              className={`character${state.character === c.id ? " selected" : ""}`}
              onClick={() => actions.setCharacter(c.id)}
            >
              <span className="portrait" aria-hidden="true">
                <CharacterImage character={c.id} file="idle.png" />
              </span>
              <span className="name">{c.name}</span>
            </button>
          ))}
        </div>
      </section>

      <section aria-labelledby="pace-label">
        <p className="label" id="pace-label">
          Pace
        </p>
        <PacePicker value={state.pace} onChange={(pace) => actions.setPace(pace)} compact />
      </section>

      <section className="switches">
        <Switch label={START_AT_LOGIN} checked={state.autostart} onChange={actions.setAutostart} />
        <Switch label="Check for updates" hint={UPDATES_HINT_SHORT} checked={state.updates} onChange={actions.setUpdates} />
      </section>

      <footer>
        {confirming ? (
          <div className="confirm" role="alert">
            <span>Forget the card position and movement order?</span>
            <button
              className="danger"
              onClick={() => {
                actions.clearData();
                setConfirming(false);
                setCleared(true);
              }}
            >
              Clear
            </button>
            <button onClick={() => setConfirming(false)}>Cancel</button>
          </div>
        ) : (
          <p className="links">
            <span>v{state.version}</span>
            <button onClick={actions.openGitHub}>GitHub</button>
            <button onClick={actions.reportProblem}>Report a problem</button>
            <button onClick={() => setConfirming(true)}>{cleared ? "Cleared" : "Clear my data"}</button>
          </p>
        )}
      </footer>
    </main>
  );
}

/** What the updates switch does, in the words of PRIVACY.md. */
export const UPDATES_HINT = "Once a day, asks GitHub for the newest version number. Nothing about you is sent.";
/** The same promise in one line, for the settings window, which has less room. */
const UPDATES_HINT_SHORT = "Once a day. Sends nothing about you.";

export function Switch({
  label,
  hint,
  checked,
  onChange,
}: {
  label: string;
  hint?: string;
  checked: boolean;
  onChange: (value: boolean) => void;
}) {
  return (
    <label className="switch-row">
      <span className="switch-text">
        {label}
        {hint && <small>{hint}</small>}
      </span>
      <input type="checkbox" role="switch" checked={checked} onChange={(e) => onChange(e.target.checked)} />
      <span className="track" aria-hidden="true" />
    </label>
  );
}
