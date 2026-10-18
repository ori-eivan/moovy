# Moovy: Tasks (v0.1)

**Status:** Planned. Nothing below is done.
**Owner key:** **Ori** = needs Ori, **Claude** = Claude Code builds it, **Both** = Claude builds, Ori checks.
**Order:** grouped by area. The build order is Ori's call. Dependencies are listed so any order stays safe.

Each task is done only when its acceptance check passes.

---

## A. Art (Ori, runs in parallel from day 1)

| # | Task | Owner | Depends on | Done when |
|---|---|---|---|---|
| A1 | Pilot character master image (front, neutral, full body) | Ori | none | Has a clear neck, arms with elbows, legs. Transparent background |
| A2 | Character sheet: name, palette (hex), personality line | Both | A1 | Written into `character.json` and the prompt kit |
| A3 | 5 state images: idle, break, celebration, water, waiting | Ori | A2 | Pass the consistency checklist in CHARACTER_PROMPT_KIT.md |
| A4 | 6 pose images | Ori | A2 | Pass the checklist **and** the pose is readable at 120 px |
| A5 | Consistency review of all 11 | Claude | A3, A4 | Side-by-side contact sheet reviewed, rejects listed by file |

## B. Foundation

| # | Task | Owner | Depends on | Done when |
|---|---|---|---|---|
| B1 | Create **private** repo `ori-eivan/moovy` | Ori approves, Claude runs | none | Repo exists with README stub and licenses. Made public only on tester day (J2), after a history check |
| B2 | Scaffold Tauri v2 + React + TypeScript | Claude | B1 | `npm run tauri dev` opens an empty window |
| B3 | Tray icon and menu (static items) | Claude | B2 | Every menu item from PRD §11 is visible, Quit works |
| B4 | Single instance | Claude | B2 | A second launch opens settings instead of a second copy |
| B5 | Store with `schemaVersion` | Claude | B2 | Settings survive restart |

## C. Scheduler (the critical code)

| # | Task | Owner | Depends on | Done when |
|---|---|---|---|---|
| C1 | Pure scheduler module (events in, decisions out) | Claude | B2 | Compiles with no Windows imports |
| C2 | Unit tests with a simulated clock | Claude | C1 | All cases in ARCHITECTURE §8 pass in `cargo test` |
| C3 | Debug speed (1 min = 1 s) | Claude | C1 | A full 60 min cycle plays in 60 s in a debug build |

## D. Windows signals

| # | Task | Owner | Depends on | Done when |
|---|---|---|---|---|
| D1 | Idle time via `GetLastInputInfo` | Claude | C1 | Tray status freezes when hands are off the keyboard |
| D2 | Lock / unlock events | Claude | C1 | Locking and unlocking resets the cycle, no card for 2 min |
| D3 | Sleep / resume (event + tick gap) | Claude | C1 | After sleep: cycle at 0, no backlog |
| D4 | Busy state (`SHQueryUserNotificationState`) | Claude | C1 | No card during a PowerPoint slideshow or a full-screen game |
| D5 | Mic in use (privacy registry) | Claude | C1 | No card during a Zoom, Teams or Meet call. Card shows after the call |

## E. Card

| # | Task | Owner | Depends on | Done when |
|---|---|---|---|---|
| E1 | Card window that never takes focus | Claude | B2 | Typing in another app while the card appears loses no keystrokes |
| E2 | Position on the active monitor's work area | Claude | E1 | Correct on 2 monitors, taskbar left / top / bottom, 100% to 200% scaling |
| E3 | Card states with placeholder art | Claude | E1, C1 | Every state in ARCHITECTURE §3.1 reachable from debug tray items |
| E4 | Timer with "Switch sides" | Claude | E3 | Two-sided movements show the cue at 50% |
| E5 | Movement content (`movements.json`) | Claude | E3 | All six movements from PRD §8, copy matches exactly |
| E6 | Rotation Movement / Stretch | Claude | C1, E5 | 8 breaks in a row follow M1 S1 M2 S2 M3 S3 M1 S1 |
| E7 | Light motion + reduced motion | Claude | E3 | Motion stops when Windows Animation effects is off |
| E8 | Card gallery (dev only): every card state and every movement side by side at real size, in a browser page | Claude | E3 | Ori can review all the designs in one page without waiting for a timer |
| E9 | Debug tray items: "Show card: <state>" for every state | Claude | E3, C3 | Every state can be opened on the real desktop in one click |

## F. Settings window

| # | Task | Owner | Depends on | Done when |
|---|---|---|---|---|
| F1 | First-run onboarding | Claude | B5, A3 (idle) | Install to "Let's move" in under 20 s, safety note visible |
| F2 | Settings: autostart, water toggle | Claude | F1 | Toggles take effect immediately and persist |
| F3 | Today and 7-day stats (local) | Claude | B5, E3 | Numbers match a manual count over one dogfood day |
| F4 | Report a problem | Claude | F1 | Opens a new GitHub issue with app version and Windows version only |

## G. Art integration

| # | Task | Owner | Depends on | Done when |
|---|---|---|---|---|
| G1 | Character pack loader with fallbacks | Claude | E3 | Deleting a pose file shows `break.png` instead of a broken card |
| G2 | Drop in Ori's 11 images | Claude | A5, G1 | Every card state shows the right image |

## H. Release

| # | Task | Owner | Depends on | Done when |
|---|---|---|---|---|
| H1 | Update notice (GitHub API) | Claude | B3 | A fake newer tag shows "Update available" in tray and settings |
| H2 | NSIS per-user installer | Claude | B2 | Installs with no admin prompt, uninstall leaves no process and no startup entry |
| H3 | CI workflow | Claude | C2 | Tests run on every push |
| H4 | Release workflow | Claude | H2 | Tag `v0.1.0` produces an installer on a GitHub Release |
| H5 | README: GIF, download, SmartScreen guide, privacy, FAQ | Both | G2, H4 | Ori reads it and approves |
| H6 | LICENSE (MIT), ASSETS_LICENSE (CC BY-NC 4.0), PRIVACY, CONTRIBUTING, SECURITY, CHANGELOG | Claude | B1 | Files present and consistent with the README |

## I. Dogfood gate (Ori, 2 to 3 days, before any tester)

Run with debug speed first, then real speed. Every line must pass:

- [ ] Read a long document for 10 minutes without input: cycle does not reset
- [ ] Lock for 5 minutes: cycle resets, no card in the first 2 minutes after unlock
- [ ] Sleep the laptop: no backlog after resume
- [ ] Zoom, Teams or Meet call: no card during the call
- [ ] PowerPoint slideshow: no card
- [ ] Typing in Word when a card appears: no lost keystrokes
- [ ] Second monitor, taskbar on the left: card fully visible on the monitor in use
- [ ] 125% laptop screen next to a 100% monitor: card sized correctly on both
- [ ] Pause 1 hour, Pause until tomorrow, Skip next: all behave as labelled
- [ ] 8 hour run: CPU near 0% when idle, memory flat (numbers recorded)
- [ ] Uninstall: nothing left running, no startup entry

## J. Testers and public gate

| # | Task | Owner | Done when |
|---|---|---|---|
| J1 | Pick 3 to 5 testers (student, designer, developer at minimum) | Ori | Names agreed |
| J2 | Send the release link and a 3-line brief | Ori | All installed |
| J3 | Collect 5-day numbers from each settings window | Ori | Numbers in one table |
| J4 | Go / no-go: at least 50% of break cards got Start | Both | Decision written in CHANGELOG |
| J5 | Public announcement | Ori | Only after J4 is a go |

## v0.2 backlog (not now)

- Microsoft Store via MSIX (plan kept outside the repo, in the project's planning folder)
- Characters 2 and 3, pack picker
- Cycle options and quiet hours
- Streak or journey
- Sound options (v0.1 is fully silent)
- macOS
