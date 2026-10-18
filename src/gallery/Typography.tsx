import { useLayoutEffect, useRef, useState, type CSSProperties } from "react";

/** Every text style in the app. Values are read from the live CSS, so this page never drifts. */
const STYLES: { key: string; name: string; where: string; sample: string }[] = [
  { key: "h1", name: "Settings title", where: "Settings window: “Moovy”", sample: "Moovy" },
  { key: "big", name: "Card headline", where: "“move!”, “Nice one!”, “Refreshed!”, “See you in 10”", sample: "Nice one!" },
  { key: "countdown", name: "Countdown", where: "Running timer", sample: "0:21" },
  { key: "title", name: "Card title", where: "Movement name, “Water break?”", sample: "Wall Angels" },
  { key: "switch", name: "Switch sides", where: "Halfway through a two-sided movement", sample: "Switch sides" },
  { key: "small", name: "Card lead-in", where: "“It's time to”", sample: "It's time to" },
  { key: "line", name: "Card body", where: "Instruction line under the title", sample: "Back to the wall, slide your arms up and down." },
  { key: "dose", name: "Dose", where: "“6 slow reps”, “15 sec per side”", sample: "15 sec per side" },
  { key: "howto", name: "How to text", where: "How to panel", sample: "Back, head and arms on the wall, elbows in a W." },
  { key: "howtoMuted", name: "How to alternative", where: "How to panel, second line", sample: "No wall? Do it standing, arms in a W." },
  { key: "btnPrimary", name: "Primary button", where: "Start, Done", sample: "Start" },
  { key: "btnSecondary", name: "Secondary button", where: "Snooze", sample: "Snooze" },
  { key: "btnLink", name: "Text link", where: "How to, Back", sample: "How to" },
  { key: "close", name: "Close", where: "× on every card", sample: "×" },
  { key: "safety", name: "Settings subtitle", where: "“Move gently. Stop if anything hurts.”", sample: "Move gently. Stop if anything hurts." },
  { key: "label", name: "Section label", where: "“Your buddy”", sample: "Your buddy" },
  { key: "charName", name: "Character name", where: "Under each character", sample: "Character 1" },
  { key: "switchRow", name: "Setting row", where: "Water reminders, Start with Windows", sample: "Water reminders" },
  { key: "footer", name: "Footer", where: "Version, GitHub, Report a problem, Clear my data", sample: "Report a problem" },
];

type Measured = {
  family: string;
  size: string;
  weight: string;
  lineHeight: string;
  letterSpacing: string;
  transform: string;
  color: string;
  background: string;
};

function toHex(rgb: string): string {
  const m = rgb.match(/\d+(\.\d+)?/g);
  if (!m || (m.length === 4 && Number(m[3]) === 0)) return "transparent";
  return "#" + m.slice(0, 3).map((n) => Number(n).toString(16).padStart(2, "0")).join("").toUpperCase();
}

function firstFamily(family: string): string {
  return family.split(",")[0].replace(/"/g, "").trim();
}

export default function Typography() {
  const probe = useRef<HTMLDivElement>(null);
  const [measured, setMeasured] = useState<Record<string, Measured>>({});
  const [fonts, setFonts] = useState<Record<string, boolean>>({});

  useLayoutEffect(() => {
    const read = () => {
      const out: Record<string, Measured> = {};
      probe.current?.querySelectorAll<HTMLElement>("[data-k]").forEach((el) => {
        const s = getComputedStyle(el);
        out[el.dataset.k!] = {
          family: s.fontFamily,
          size: s.fontSize,
          weight: s.fontWeight,
          lineHeight: s.lineHeight,
          letterSpacing: s.letterSpacing,
          transform: s.textTransform,
          color: s.color,
          background: s.backgroundColor,
        };
      });
      setMeasured(out);
      setFonts({
        "Clash Display": document.fonts.check('600 20px "Clash Display"') && [...document.fonts].some((f) => f.family.includes("Clash") && f.status === "loaded"),
        "Hyperlegible Sans": [...document.fonts].some((f) => f.family.includes("Hyperlegible") && f.status === "loaded"),
        Chillax: [...document.fonts].some((f) => f.family.includes("Chillax") && f.status === "loaded"),
        Manrope: [...document.fonts].some((f) => f.family.includes("Manrope") && f.status === "loaded"),
      });
    };
    read();
    document.fonts.ready.then(read);
  }, []);

  return (
    <section id="typography">
      <h2>
        <span className="step">Part 3</span> Typography
      </h2>

      <div className="font-cards">
        <FontCard
          family="Clash Display"
          role="Headings: card titles, card headlines, settings title"
          license="ITF Free Font License. Local only, not in the repo"
          loaded={fonts["Clash Display"]}
        />
        <FontCard
          family="Chillax"
          role="Attention: buttons (bold), Switch sides, dose"
          license="ITF Free Font License. Local only, not in the repo"
          loaded={fonts["Chillax"]}
        />
        <FontCard
          family="Manrope"
          role="Long descriptions: card body, How to, settings subtitle"
          license="SIL Open Font License. Bundled in the repo"
          loaded={fonts["Manrope"]}
        />
        <FontCard
          family="Hyperlegible Sans"
          role="Everything else: labels, timer, settings rows"
          license="SIL Open Font License. Bundled in the repo"
          loaded={fonts["Hyperlegible Sans"]}
        />
      </div>

      <table className="type-table">
        <thead>
          <tr>
            <th>Sample</th>
            <th>Style</th>
            <th>Font</th>
            <th>Size</th>
            <th>Weight</th>
            <th>Line</th>
            <th>Colour</th>
          </tr>
        </thead>
        <tbody>
          {STYLES.map((st) => {
            const m = measured[st.key];
            if (!m) return null;
            const style: CSSProperties = {
              fontFamily: m.family,
              fontSize: m.size,
              fontWeight: m.weight as CSSProperties["fontWeight"],
              lineHeight: m.lineHeight,
              letterSpacing: m.letterSpacing,
              textTransform: m.transform as CSSProperties["textTransform"],
              color: m.color,
            };
            const filled = toHex(m.background) !== "transparent";
            return (
              <tr key={st.key}>
                <td className="sample">
                  <span style={{ ...style, ...(filled ? { background: m.background, padding: "4px 12px", borderRadius: 999 } : {}) }}>
                    {st.sample}
                  </span>
                </td>
                <td>
                  <b>{st.name}</b>
                  <small>{st.where}</small>
                </td>
                <td>{firstFamily(m.family)}</td>
                <td className="num">{m.size}</td>
                <td className="num">{m.weight}</td>
                <td className="num">{m.lineHeight === "normal" ? "normal" : `${Math.round(parseFloat(m.lineHeight))}px`}</td>
                <td>
                  <span className="swatch" style={{ background: m.color }} /> {toHex(m.color)}
                  {filled && (
                    <small>
                      on <span className="swatch" style={{ background: m.background }} /> {toHex(m.background)}
                    </small>
                  )}
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>

      {/* Hidden copies of the real elements, in their real places in the markup, to read the live CSS. */}
      <div ref={probe} aria-hidden="true" className="type-probe">
        <div className="card view-movement howto-open">
          <div className="body">
            <p className="small" data-k="small">x</p>
            <p className="big" data-k="big">x</p>
            <p className="title" data-k="title">x</p>
            <p className="line" data-k="line">x</p>
            <p className="dose" data-k="dose">x</p>
            <p className="switch" data-k="switch">x</p>
            <p className="countdown" data-k="countdown">x</p>
            <div className="actions">
              <button className="primary" data-k="btnPrimary">x</button>
              <button data-k="btnSecondary">x</button>
              <button className="link" data-k="btnLink">x</button>
            </div>
          </div>
          <div className="howto-pane">
            <p data-k="howto">x</p>
            <p className="muted" data-k="howtoMuted">x</p>
          </div>
          <button className="close" data-k="close">x</button>
        </div>
        <main className="settings">
          <h1 data-k="h1">x</h1>
          <p className="safety" data-k="safety">x</p>
          <p className="label" data-k="label">x</p>
          <button className="character">
            <span className="name" data-k="charName">x</span>
          </button>
          <label className="switch-row">
            <span data-k="switchRow">x</span>
          </label>
          <p className="links">
            <button data-k="footer">x</button>
          </p>
        </main>
      </div>
    </section>
  );
}

function FontCard({ family, role, license, loaded }: { family: string; role: string; license: string; loaded?: boolean }) {
  return (
    <div className="font-card">
      <p className="font-name" style={{ fontFamily: `"${family}"` }}>
        {family}
      </p>
      <p className="font-glyphs" style={{ fontFamily: `"${family}"` }}>
        Aa Bb Cc 0123456789
      </p>
      <p>{role}</p>
      <small>{license}</small>
      <small className={loaded ? "ok" : "bad"}>{loaded ? "Loaded" : "Not loaded: showing the fallback"}</small>
    </div>
  );
}
