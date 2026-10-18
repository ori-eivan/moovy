# Moovy: Design Brief for Claude Design

Paste this whole brief into Claude Design. It describes every screen to design, with exact sizes, states and copy, so the result maps 1:1 to the app.

---

## Context

Moovy is a free, open-source Windows tray app. Every 30 minutes of active computer work, a small card appears at the bottom-left of the screen, alternating between a water reminder and a one-minute movement break. A friendly original mascot appears on every card. Then it gets out of the way.

Tone: playful, warm, calm. Never urgent, never guilt. Think "a friend tapping your shoulder", not "a system warning".

Reference spirit: compact rounded notification cards with a mascot sticker on the left and short, friendly copy on the right. Original design only. Do not copy any existing brand.

## What to design

1. **The card**, in 8 states (section A)
2. **The settings window**, in 3 states (section B)
3. **Design tokens** (section C)

Not to design: the tray menu (it's a native Windows menu) and the character art (made separately; use a placeholder square).

## Global rules

- English only. Left-to-right.
- The card floats over any wallpaper or app, light or dark, so it needs a clear edge: a 1px border and one soft shadow.
- Light theme only for v0.1.
- One font family, and it must be licensed SIL OFL (for example Nunito, DM Sans, Plus Jakarta Sans, Outfit) so the open-source app can bundle it.
- Minimum text size 13px. Body text contrast at least 4.5:1.
- Click targets at least 32px tall.
- Visible keyboard focus ring on buttons.
- No purple gradients, no glassmorphism, no emoji as icons.
- Everything must still read well with motion turned off.

---

## A. The card

**Size:** width 360px fixed. Height fits the content: about 132px collapsed, up to 320px with "How to" open.
**Layout:** mascot area on the left (112 × 112px, rounded, soft tinted background), text and buttons on the right, close X in the top-right corner (24px hit area, subtle).
**Mascot placeholder:** a rounded square labelled with the image file name (e.g. `break.png`).

Design each state as its own frame:

### A1. Break intro (shown about 3 seconds)
- Mascot: `break.png`
- Small line: "It's time to"
- Big highlighted word: "move!" (on Stretch turns: "stretch!"), with a playful accent (a hand-drawn underline or squiggle)
- Small line: "Quick one while you're here."
- No buttons except X

### A2. Movement (main view)
- Mascot: the movement pose, e.g. `wall-angels.png`
- Title: "Wall Angels"
- Top-right, small and muted: "2 today"
- Instruction line: "Back to the wall, slide your arms up and down."
- Dose line, in the accent colour: "10 slow reps"
- Bottom row: **Start** (primary, dark filled pill), **Snooze** (secondary, light pill), "How to ⌄" (text link, right), small duration "60 sec"

### A3. Movement with "How to" open
- Same as A2, plus an expanded block under the instruction:
  - "Stand with your back, head and arms against a wall, elbows bent in a W. Slowly slide your arms up, then back down."
  - Muted: "No wall? Do it standing, arms in a W."
- Card grows taller. Buttons stay at the bottom.

### A4. Running (timer)
- Mascot: the movement pose
- Title: "Wall Angels"
- A big, calm countdown: "0:42"
- A slim progress bar or ring
- Only X remains (no Start or Snooze)

### A5. Running, switch sides
- Same as A4 for a two-sided move (e.g. "Hip Flexor Stretch", "0:30")
- A clear but gentle "Switch sides" label replaces the subtitle for a few seconds. There is no sound, so this must be visible from a step away.

### A6. Celebration (about 4 seconds)
- Mascot: `celebration.png`
- Big: "Nice one!"
- Small: "3 today"
- No buttons except X

### A7. Water
- Mascot: `water.png`
- Title: "Water break?"
- Line: "A few sips is plenty."
- Buttons: **Done** (primary), **Snooze** (secondary)

### A8. Waiting (after Snooze, about 2 seconds)
- Mascot: `waiting.png`
- Big: "See you in 10"
- No buttons

---

## B. Settings window

**Size:** 380 × 600px, portrait, one window, no tabs. Title bar: "Moovy".

### B1. First run (onboarding)
- Mascot large and centred: `idle.png` (160 × 160)
- Headline: "Hi, I'm [Name]." (placeholder name)
- Line: "Every 30 minutes I'll pop in with a sip of water or a one-minute move."
- Safety note in a soft callout: "Move gently. Stop if anything hurts."
- Checkbox (checked): "Start Moovy with Windows"
- Primary button, full width: "Let's move"
- Tiny footer: "Private. Local. No account."

### B2. Settings
- Small mascot `idle.png` (72 × 72) beside "Moovy" and status "Next: water in 12 min"
- Section "Today": four small stats: Cards 6 · Started 3 · Done 3 · Water 4
- Section "Last 7 days": "You started 58% of your breaks." with a tiny 7-bar chart
- Section "Settings": toggle "Start with Windows", toggle "Water reminders"
- Buttons: "Pause for 1 hour", "Pause until tomorrow"
- Footer: version "v0.1.0" · "GitHub" · "Report a problem"
- An "Update available: v0.1.1" pill near the version (show it in one variant)

### B3. Paused
- Same as B2, but the status reads "Paused until 14:30" with a "Resume" button replacing the pause buttons

---

## C. Tokens to hand back

When the design is done, list these so they can be copied into code:

- Colours (hex): background, surface, border, text, muted text, primary button, primary button text, secondary button, accent (highlight word and dose), mascot tile tint, progress bar
- Font family and weights used
- Font sizes for: big word, title, body, small, countdown
- Radii: card, mascot tile, buttons
- Shadow (one value)
- Spacing: card padding, gap between mascot and text, gap between buttons

Export: share the Claude Design link, or export the frames as HTML/CSS.
