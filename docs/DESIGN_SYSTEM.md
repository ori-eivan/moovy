# Moovy: Design System

The single source of truth for how Moovy looks and moves. Read it before any UI change; update it in the same change when a decision changes. This project has its own design language, so the general ORIKU visual system does not apply here.

**Last updated:** 2026-10-20
**Live check:** `http://localhost:1420/gallery.html` → Part 3, Typography. It reads every text style from the running CSS, so it shows what is really on screen.

`DESIGN_BRIEF.md` is the original brief for the first design pass. Where the two disagree, this file wins (the brief asked for one font and no glass; both were changed on purpose, see below).

---

## 1. Type: four fonts, four jobs

Every text element gets one of these roles **explicitly**, with its own `font-family`. Nothing relies on inheriting the window default, because the default is the label font: that is how the pace names ended up in the wrong font in 0.1.2.

| Role | Font | Used for | Weight |
|---|---|---|---|
| **Title** | Clash Display | Window title "Moovy", onboarding questions, card titles ("Wall Angels", "Water break?"), card headline ("move!", "Nice one!"), names of choices (pace names, character names) | 500 to 600 |
| **Action** | Chillax | Every button: Start, Done, Snooze, Next/Close, Clear, Cancel; "Switch sides" | 600 |
| **Description** | Manrope | Sentences and details: card instruction lines, How to text, onboarding body text, pace details ("water 30 · move 60"), the hello bubble | 400 to 600 |
| **Label** | Hyperlegible Sans | Section labels ("YOUR BUDDY", "PACE"), the step counter, setting rows, tags ("Best to start"), footer, the countdown and other numbers that must be read at a glance | 400 to 700 |

Rules:
- **Numbers inside sentences use Manrope.** Hyperlegible Sans draws a slashed zero (0 with a line through it). Good on the countdown, wrong in "water 30 · move 60".
- **The countdown stays in Hyperlegible Sans**: Chillax has no tabular digits, so the timer would jitter.
- Clash Display and Chillax are under the ITF Free Font License: they are not in the repository (see README, Fonts). Without them the UI falls back to Hyperlegible Sans, so always list the fallback: `"Clash Display", "Hyperlegible Sans", "Segoe UI", sans-serif`.
- Minimum text size 13 px. Exception: the small "Best to start" tag (11 px, bold, on a solid colour).

## 2. Colour

Light theme only. Warm paper and ink, one accent.

| Token | Hex | Used for |
|---|---|---|
| Ink | `#221D1A` | Text, primary buttons, selected outlines, the bubble |
| Body | `#4A413B` | Long descriptions |
| Muted | `#6B6058` | Secondary text, details, Back links |
| Faint | `#8A7F74` | Section labels, hints, footer |
| Paper | `#FFFDF8` | Card and window background |
| Tint | `#F2ECE2` | Mascot tile, secondary buttons |
| Hover | `#F8F3EB` | Hover on tiles |
| Selected | `#FAF6EF` | Background of a selected choice |
| Line | `#EEE6DA` | Dividers |
| Line strong | `#E6DDD0` | Choice row borders |
| Control | `#D9D1C5` / `#B9ADA0` | Switch track off, radio ring, hover border |
| Accent | `#C4502C` | Dose line, "move!", tags, focus ring |
| Danger | `#B3412A` | "Clear" (destructive) |

No other colours without adding them here first. Character art brings its own colours; the UI around it stays in this palette.

## 3. Shape and depth

| Thing | Value |
|---|---|
| Card | radius 18 px, border `1px rgba(34,29,26,0.12)`, shadow `0 4px 10px rgba(0,0,0,0.18)` |
| Mascot tile, character tile | radius 14 px, Tint background |
| Choice row (pace) | radius 14 px, 1.5 px Line strong border; selected: Ink border + Selected background |
| Buttons | pill (radius 999 px), at least 30 px tall (40 px in onboarding) |
| Bubble | radius 12 px, Ink background, small rotated-square tail |

Click targets at least 32 px tall. Visible focus ring: 2 px Accent outline, 2 px offset.

## 4. Sizes

| Surface | Size |
|---|---|
| Card window | 380 px wide (card 360 + shadow room), height follows the content |
| Settings / onboarding window | 380 × 560 px. Everything must fit without scrolling, including the "Clear my data" confirmation |
| Mascot on the card | 126 px tile |

## 5. Motion

Calm and short. Everything turns off with Windows "Animation effects" off (`prefers-reduced-motion`), and every screen must still make sense without motion.

| Where | What | Timing |
|---|---|---|
| Onboarding screen change | **Frosted glass** (GSAP): the screen blurs, fades and shrinks a little under a sheet of frosted glass; the next one comes into focus as the glass clears. Back runs it in the other direction | hold 0.18 s after the tap; out 0.30 s `power2.in`, blur 10 px; in 0.45 s `power3.out`, blur 12 px → 0 |
| Hello bubble | Pops in after the card | 0.35 s `back.out(1.6)`, 0.25 s delay |
| Card: water Done, celebration | Small bounce (CSS) | 600 ms ease-out |
| Card: Waiting | Gentle sway (CSS) | 1600 ms loop |
| How to panel | Slides in with clip-path (CSS) | 300 ms in, 460 ms out |

Glass is used **only** for onboarding screen changes. It is not a general style: no glass cards, no glass buttons.

GSAP (`gsap` 3.14.2) is for multi-step sequences like the glass transition. Single-property effects stay in CSS.

## 6. Components

| Component | Where it lives |
|---|---|
| Card (all states) | `src/card/CardView.tsx`, `src/card/card.css` |
| Onboarding | `src/settings/Onboarding.tsx`, `src/settings/onboarding.css` |
| Settings window | `src/settings/SettingsView.tsx`, `src/settings/settings.css` |
| Choice rows (pace) | `.paces` / `.pace` in `onboarding.css`, used by onboarding and settings |
| Pace data | `src/content/paces.ts` (ids must match `Pace` in `scheduler.rs`) |

## 7. Copy

English only. Short, warm, never urgent or guilty ("Quick one while you're here", not "You missed a break"). No medical claims. Buttons are verbs.

## 8. Before you ship a UI change

- [ ] Every new text element has an explicit role from section 1
- [ ] Colours come from section 2
- [ ] The window still fits 380 × 560 with no scrolling
- [ ] Checked in the gallery, including Part 3, Typography
- [ ] Works with reduced motion
- [ ] This file updated if a decision changed
