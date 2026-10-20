import { DEFAULT_PACE, MOVE_RANGE, PRESETS, STEP, WATER_RANGE, describe, every, type Pace } from "../content/paces";

interface Props {
  value: Pace;
  /** `kind` tells onboarding whether to move on (a preset) or wait for Next (custom). */
  onChange: (pace: Pace, kind: "preset" | "custom") => void;
  /** Settings window: one line per preset. */
  compact?: boolean;
}

/**
 * The pace choice (decided 2026-10-20): two presets and Custom. Only Custom opens the sliders.
 * Each option shows its minutes as two small boxes: 💧 water and 🚶 move.
 */
export default function PacePicker({ value, onChange, compact = false }: Props) {
  const custom = value.id === "custom";
  // The slider's leftmost stop, one step below the range, means "no water".
  const waterOff = WATER_RANGE.min - STEP;

  return (
    <div className={`paces${compact ? " compact" : ""}`} role="radiogroup" aria-label="Pace">
      {PRESETS.map((p) => (
        <button
          key={p.id}
          role="radio"
          aria-checked={value.id === p.id}
          className={`pace${value.id === p.id ? " selected" : ""}`}
          onClick={() => onChange({ id: p.id, waterMinutes: p.waterMinutes, moveMinutes: p.moveMinutes }, "preset")}
        >
          <span className="radio" aria-hidden="true" />
          {compact ? (
            <>
              <span className="pace-title">{p.title}</span>
              <Chips pace={p} />
            </>
          ) : (
            <span className="pace-stack">
              <span className="pace-title">
                {p.title}
                {p.id === DEFAULT_PACE.id && <span className="tag">Best to start</span>}
              </span>
              <Chips pace={p} />
            </span>
          )}
        </button>
      ))}

      <div className={`pace custom-pace${custom ? " selected open" : ""}`}>
        <button
          className="custom-head"
          role="radio"
          aria-checked={custom}
          onClick={() => onChange({ ...value, id: "custom" }, "custom")}
        >
          <span className="radio" aria-hidden="true" />
          <span className="pace-title">Custom</span>
          {custom ? <Chips pace={value} /> : <span className="pace-short">Set your own</span>}
        </button>
        {custom && (
          <div className="sliders">
            <Slider
              label="Water"
              icon="droplet"
              min={waterOff}
              max={WATER_RANGE.max}
              value={value.waterMinutes ?? waterOff}
              readout={value.waterMinutes === null ? "Off" : every(value.waterMinutes)}
              off={value.waterMinutes === null}
              onChange={(m) => onChange({ ...value, waterMinutes: m === waterOff ? null : m }, "custom")}
            />
            <Slider
              label="Move"
              icon="person-walking"
              min={MOVE_RANGE.min}
              max={MOVE_RANGE.max}
              value={value.moveMinutes}
              readout={every(value.moveMinutes)}
              onChange={(m) => onChange({ ...value, moveMinutes: m }, "custom")}
            />
          </div>
        )}
      </div>
    </div>
  );
}

/** Microsoft Fluent Emoji (Flat), MIT: public/icons/FLUENT-EMOJI-LICENSE.txt. Same look on Windows and Mac. */
function Icon({ name }: { name: "droplet" | "person-walking" }) {
  return <img className="pace-icon" src={`/icons/${name}.svg`} alt="" aria-hidden="true" draggable={false} />;
}

/** 💧 45′  🚶 60′ */
function Chips({ pace }: { pace: Pace }) {
  return (
    <span className="chips" title={describe(pace)} aria-label={describe(pace)}>
      <span className={`chip water${pace.waterMinutes === null ? " off" : ""}`}>
        <Icon name="droplet" />
        {pace.waterMinutes === null ? "off" : `${pace.waterMinutes}′`}
      </span>
      <span className="chip move">
        <Icon name="person-walking" />
        {`${pace.moveMinutes}′`}
      </span>
    </span>
  );
}

function Slider(props: {
  label: string;
  icon: "droplet" | "person-walking";
  min: number;
  max: number;
  value: number;
  readout: string;
  off?: boolean;
  onChange: (minutes: number) => void;
}) {
  const fill = ((props.value - props.min) / (props.max - props.min)) * 100;
  return (
    <div className={`slider${props.off ? " off" : ""}`}>
      <div className="slider-head">
        <span className="slider-label">
          <Icon name={props.icon} />
          {props.label}
        </span>
        <span className="slider-readout">{props.readout}</span>
      </div>
      <input
        type="range"
        min={props.min}
        max={props.max}
        step={STEP}
        value={props.value}
        aria-label={props.label}
        aria-valuetext={props.readout}
        style={{ ["--fill" as string]: `${fill}%` }}
        onChange={(e) => props.onChange(Number(e.target.value))}
      />
    </div>
  );
}
