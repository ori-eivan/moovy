# Moovy: Product Requirements (v0.1)

**Product:** Moovy by Ori Eivan
**Status:** Planned, not implemented. Nothing in this document exists yet.
**Platform:** Windows 10/11 (v0.1). macOS later.
**Language:** English only.
**License:** Code MIT. Character art CC BY-NC 4.0.
**Last updated:** 2026-10-17

---

## 1. One line

A small character pops up in the corner of your screen every half hour, alternating between a glass of water and a one-minute movement, and then gets out of your way.

## 2. Goal

A public GitHub repository good enough for Ori Eivan's portfolio, used daily by Ori, and validated by 3 to 5 outside testers before it is announced publicly.

**Not a goal for v0.1:** reach, downloads, monetisation, medical outcomes.

## 3. Principles

1. **One action.** Every card can be resolved with one click.
2. **Never trapped.** Every card has a quiet way out (X), and the tray can pause Moovy.
3. **Never rude.** Moovy never takes keyboard focus, never shows during a call or presentation, and never piles up cards.
4. **No guilt.** No streaks, no sad character, no "you missed" messages.
5. **No medical claims.** Cards describe what to do and how it feels, never what it fixes.
6. **Private.** Everything stays on the device. v0.1 makes no network calls at all.
7. **Small.** Tray app, low CPU and memory when idle.

## 4. Users

Developers, designers, students and remote workers who sit at a Windows computer for hours and lose track of time.

## 5. The core loop

Moovy counts **active computer time only**. Water and moves each have their **own interval** (decided 2026-10-20):

| Pace | 💧 Water | 🚶 Move |
|---|---|---|
| Recommended (default) | every 45 min | every 60 min |
| Active | every 30 min | every 45 min |
| Custom | off, or every 20 to 120 min | every 30 to 120 min |

Custom minutes move in steps of 5, with two sliders that appear only when Custom is picked. A move always outranks water when both are due, and the 15-minute minimum gap between cards still applies.

Settings from older versions are kept: the old Recommended (30/60) becomes the new Recommended, "Relaxed" becomes Custom 45/90, and water off becomes Custom with water off.

## 6. Cards

All cards appear at the **bottom-left of the work area** of the monitor you are working on, without taking focus.

### 6.1 Break card

1. Appears in the **Break** state: headline "It's time to move!" (or "It's time to stretch!" for a Stretch turn) with the Break illustration.
2. After about 3 seconds it turns into the **movement view** in the same card: movement pose illustration, name, one-line instruction, reps or duration, daily count ("2 today").
3. Buttons: **Start** (timed movements) or **Done** (rep-based movements), **Snooze**, **How to** (opens the instructions panel), **X** (skip silently).
4. **Rep-based** movements (Wall Angels, Spine Twist) have no timer: **Done** closes the card and counts the break as completed.
   **Timed** movements (the stretches) are all 30 seconds: **Start** runs one timer for both sides, and "Switch sides" appears at 15 seconds.
5. When the timer ends: **Celebration** state, short message, card hides after about 4 seconds.

### 6.2 Water card

- Drink Water illustration, headline "Water break?", line "A few sips is plenty."
- Buttons: **Done**, **Snooze**, **X**.
- Done: small happy bounce, card hides. No timer.

### 6.3 Snooze

- The card shows the **Waiting** state for about 2 seconds ("See you in 10"), then hides.
- The same card returns after 10 minutes.
- If that would put it less than 15 minutes before the next card, the next card moves later.

### 6.4 X (skip)

- Card hides immediately. Nothing is counted against the user. The cycle continues.

## 7. Scheduling rules

| Situation | Behaviour |
|---|---|
| No keyboard or mouse input | Counting **pauses**. It does not reset. |
| Away 15+ minutes, screen locked, or computer asleep | Both clocks **restart** (being away counts as a break). The movement rotation continues where it stopped |
| Any two cards | At least 15 minutes apart |
| Windows reports busy (presentation, full-screen game, quiet time) | Card waits |
| Microphone in use (a call) | Card waits |
| Sharing the screen, a window or a tab: Chrome or Edge (English or Hebrew UI), or the Zoom app | Card waits |
| Screen sharing or recording of any kind | The card is left out of the capture; only the user sees it |
| First 2 minutes after unlock or wake | Card waits |
| A card has waited more than 20 minutes | It is dropped. The cycle continues |
| Waiting cards | Never more than one in the queue |

## 8. The six movements

Breaks alternate: Movement, Stretch, Movement, Stretch. Within each category the three rotate in order.

Card copy rule: say what to do and how it feels. No claims of improvement.

### Movement

| ID | Name | Dose | Card line | How to | Alternative |
|---|---|---|---|---|---|
| `wall-angels` | Wall Angels | 6 slow reps (30s) | Back to the wall, slide your arms up and down. | Back, head and arms on the wall, elbows in a W. Slide your arms up, then down. | No wall? Do it standing, arms in a W. |
| `spine-twist` | Standing Spine Twist | 5 per side (30s) | Stand tall and turn gently side to side. | Feet hip-width, arms crossed. Turn your upper body side to side, hips facing forward. | Seated version works too. |
| `hip-flexor` | Hip Flexor Stretch | 15s per side (30s) | Step back and feel the front of your hip open. | Split stance, one foot back. Tuck your hips under and lean forward gently. | Hold a chair or desk for balance. |

### Stretch

| ID | Name | Dose | Card line | How to | Alternative |
|---|---|---|---|---|---|
| `neck-flexion` | Neck Flexion Stretch | 30s | Hands rest behind your head, chin to chest. | Hands rest lightly behind your head. Let your chin drop to your chest. Don't pull. | Without hands is fine. |
| `upper-trap` | Upper Trap Stretch | 15s per side (30s) | One hand behind your back, tilt your head away. | One hand behind your back. Tilt your head gently to the other shoulder. Don't pull. | Or hold the seat of your chair. |
| `doorway-chest` | Doorway Stretch | 15s per side (30s) | Forearm on a door frame, lean forward a little. | Forearm on a door frame, elbow at shoulder height. Step through gently. | No doorway? Clasp your hands behind your back. |

No safety line in the UI (removed 2026-10-18). The copy stays gentle ("Don't pull", no forced positions) instead.

## 9. Character and illustrations

- v0.1 ships **one** original character by Ori: **Thomas**, a tabby cat (the default). Art under CC BY-NC 4.0, see `ASSETS_LICENSE`. A character must have a clear neck, arms with elbows, and legs.
- **11 illustrations**, transparent PNG, consistent style:

| File | Used for |
|---|---|
| `idle.png` | Onboarding and settings window |
| `break.png` | Break card intro |
| `celebration.png` | End of timer |
| `water.png` | Water card (holding a fresh glass of water) |
| `waiting.png` | After Snooze |
| `wall-angels.png`, `spine-twist.png`, `hip-flexor.png` | Movement poses |
| `neck-flexion.png`, `upper-trap.png`, `doorway-chest.png` | Stretch poses |

- Light motion in code only: gentle breathing on idle states, bounce on celebration and water Done, small sway on Waiting. Off when Windows "Animation effects" is off.
- Characters are character packs (see ARCHITECTURE.md). The settings window offers every pack that ships with the app.

## 10. Settings window (minimal, decided 2026-10-18)

**One window, one screen, no separate onboarding.** A regular window in the centre of the screen. It opens by itself on first launch and from "Settings" in the tray.

- Title "Moovy"
- **Your buddy:** pick a character. Picking one shows a card with that character right away, so the choice is seen.
- **Water reminders** (switch, on by default)
- **Start with Windows** (switch, on by default, switched on at first launch)
- Footer: version · GitHub · Report a problem · Clear my data (asks to confirm; forgets card position and movement order, keeps settings)

Deliberately not in the window: stats, pause controls, interval, sounds.

## 11. Tray menu

Kept minimal (decided 2026-10-18):

- Next card in N min (status line)
- Settings
- Copy my stats
- Set card position (shows the card with a "drag me anywhere" bubble; letting go saves the spot)
- Update available: vX.Y.Z (shown only when there is one)
- Quit

Development builds also show "Fast mode". Pause and skip were removed from the tray; the scheduler still supports them and they can come back in v0.2.

## 12. Privacy

- No account, no telemetry, no analytics.
- Settings and daily counts are stored locally in the user's AppData.
- Microphone: Moovy checks only **whether** some app is recording from the microphone, by asking Windows' audio system and its privacy records. It never opens, records or listens to the microphone.
- Network: none in v0.1 (the update check is not built yet). See `PRIVACY.md`, which is the user-facing version of this section.
- Screen sharing: window titles are read and compared to recognise a share bar, never stored.
- Uninstall removes the app. A clear note explains where local data lives and how to delete it.

## 13. Success metric and release gate

- **Day 7:** GitHub release sent to 3 to 5 testers (a student, a designer, a developer at minimum).
- **Testers use it for 5 working days.**
- **Public announcement** only if, across testers, **at least 50% of break cards get Start**. The settings window shows no stats (decided 2026-10-18). Moovy counts silently in its local file, and testers use **Copy my stats** in the tray to copy a one-line 7-day summary and send it themselves. Nothing is sent automatically.
- If under 50%, the problem is the idea or the card, not the code. Revisit before announcing.

## 14. Out of scope for v0.1

- Microsoft Store / MSIX (v0.2)
- Self-updating (v0.2, through the Store)
- Streaks, levels, journey
- Interval settings, quiet hours
- Additional characters
- Rive, video or frame animation
- macOS, Linux, hardware companion
- Languages other than English

## 15. Acceptance criteria (product level)

1. First run to "Let's move" in under 20 seconds.
2. Every card is resolvable in one click.
3. No card ever takes keyboard focus.
4. No card appears while the microphone is in use, during a presentation, or in a full-screen game.
5. Never two cards within 15 minutes.
6. Reading without input for 10 minutes does not reset the cycle.
7. After sleep, lock or 15+ minutes away, the clock restarts with the water/break order kept, and no backlog of cards appears.
8. Settings and daily counts survive restart.
9. The card always appears fully on screen, on the monitor in use, with the taskbar on any side and at 100% to 200% scaling.
10. Idle CPU near 0% and no memory growth over an 8 hour run (measured, not assumed).
11. Uninstall leaves no running process and no startup entry.

## 16. Decisions closed on 2026-10-17

| Topic | Decision |
|---|---|
| Code license | MIT |
| Character art license | CC BY-NC 4.0 (share and adapt with credit, no commercial use) |
| Sound | Silent. No sounds at all in v0.1 |
| Repository | `ori-eivan/moovy`, private while building, public on the day it goes to testers. History checked for personal data before it goes public |
| Cycle | Fixed 60 minutes in v0.1. The water toggle is the only frequency setting |
| Pilot character name | Open. Code starts with a placeholder character |
