# Moovy: Architecture (v0.1)

**Status:** Planned, not implemented.
**Stack:** Tauri v2 (Rust host) + React + TypeScript (UI).
**Target:** Windows 10/11, x64.

Items marked **[verify]** must be checked against current docs or on a real machine before they are relied on.

---

## 1. Shape of the app

```text
┌──────────────────────────── Rust host (always running) ───────────────────────────┐
│                                                                                   │
│  ActivityMonitor ──signals──▶ Scheduler (pure) ──decisions──▶ CardController      │
│   idle seconds                 cycle, gaps,                    position, show,    │
│   lock / unlock                queue, snooze,                  hide, no focus     │
│   sleep / resume               pause, drop                                         │
│   busy state                                                                      │
│   mic in use                                                                      │
│                                                                                   │
│  Store (settings + daily counts)    Tray    UpdateChecker    single-instance       │
└───────────────────────────────────────────────────────────────────────────────────┘
                 │ events                              ▲ user actions
                 ▼                                     │
┌──────────────── WebView windows (React) ────────────────┐
│  card window:     Break → Movement → Running → Celebration │
│                   Water → Done                          │
│                   Waiting                               │
│  settings window: onboarding + settings                 │
└─────────────────────────────────────────────────────────┘
```

**Rule:** the Rust host decides **when**. The React card decides **how it looks**. The character pack decides **what it looks like**. None of them reach into the others.

## 2. Modules

### 2.1 Scheduler (pure Rust, no OS calls)

The heart of the product and the riskiest part. It is a plain state machine with no clock and no Windows calls of its own, so it can be unit-tested against a simulated clock.

**Input:** a stream of events with timestamps.

```rust
enum Event {
    Tick { now_ms: u64, idle_ms: u64 },   // every 1s
    Locked, Unlocked,
    Suspended, Resumed,
    BusyChanged(bool),                    // presentation / fullscreen / quiet time
    MicInUse(bool),
    UserAction(CardAction),               // start, done, snooze, skip, timer_finished
    PauseFor(Duration), PauseUntilTomorrow, Resume, SkipNext,
    SettingsChanged { water_enabled: bool },
}
```

**Output:** decisions.

```rust
enum Decision {
    ShowCard(CardKind),   // Water | Break(MovementId)
    HideCard,
    UpdateTray(TrayStatus),
}
```

**Rules (from PRD section 7):**

| Constant | Value |
|---|---|
| `CYCLE` | 60 min active |
| `WATER_AT` | 30 min active |
| `IDLE_PAUSE_AFTER` | 2 min without input: stop counting |
| `AWAY_RESET_AFTER` | 15 min without input: reset cycle |
| `MIN_GAP` | 15 min between any two cards |
| `SNOOZE` | 10 min |
| `MAX_WAIT` | 20 min, then drop the queued card |
| `GRACE_AFTER_UNLOCK` | 2 min |
| `QUEUE_MAX` | 1 |

**Sleep detection without trusting timers:** each `Tick` carries wall time. If the gap since the previous tick is more than 30 seconds, treat it as a sleep and reset the cycle. This also covers a missed `Resumed` event.

**Rotation:** `next_category` alternates Movement and Stretch. Each category keeps its own index over its three movements. Both persist in the Store.

**Debug speed:** a `DEBUG_SPEED` multiplier (1 min = 1 s) available only in debug builds and from a hidden tray item while dogfooding.

### 2.2 ActivityMonitor (Rust, Windows only)

Turns Windows state into `Event`s. One background thread, polling once a second plus window messages.

| Signal | Source | Notes |
|---|---|---|
| Idle time | `GetLastInputInfo` | Standard and cheap |
| Lock / unlock | `WTSRegisterSessionNotification` → `WM_WTSSESSION_CHANGE` | **[verify]** on a hidden message window inside Tauri |
| Sleep / resume | `WM_POWERBROADCAST`, plus the tick gap rule | Gap rule is the safety net |
| Busy | `SHQueryUserNotificationState`: only `QUNS_ACCEPTS_NOTIFICATIONS` counts as free | **[verify]** whether Windows 11 Focus / Do Not Disturb is reflected |
| Mic in use | Registry `HKCU\Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\microphone` (and `NonPackaged` subkeys): `LastUsedTimeStop == 0` means in use | Reads Windows' privacy record only. Never opens the mic. **[verify]** on Windows 10 and 11 |

Uses the `windows` crate. No admin rights needed.

### 2.3 CardController (Rust)

- One pre-created card window, hidden when not in use: `decorations: false`, `transparent: true`, `skipTaskbar: true`, `alwaysOnTop: true`, `resizable: false`, `focus: false`.
- Extra Win32 style `WS_EX_NOACTIVATE`, and shown with `SW_SHOWNOACTIVATE`, so it never steals keyboard focus. **[verify]** that clicks on buttons still register.
- **Position:** at show time, take the monitor that holds the foreground window (`MonitorFromWindow(GetForegroundWindow())`), read its work area (`GetMonitorInfo`, `rcWork`, which excludes the taskbar), and place the card 16 px from the left and bottom of that work area. Use physical pixels and the monitor's DPI scale.
- The card ignores keyboard input until it has been clicked.

### 2.4 Store

`tauri-plugin-store`, one JSON file in AppData:

```json
{
  "schemaVersion": 1,
  "settings": { "waterEnabled": true, "autostart": true, "onboarded": true },
  "rotation": { "next": "movement", "movementIndex": 1, "stretchIndex": 0 },
  "days": {
    "2026-10-24": { "shown": 6, "started": 3, "completed": 3, "snoozed": 1, "skipped": 1, "waterDone": 4 }
  }
}
```

Days older than 30 are pruned. The cycle position is **not** persisted: a restart means a fresh cycle, which is correct after a reboot.

### 2.5 Tray

Tauri v2 tray icon and menu (PRD section 11). The status line ("Next card in 12 min") updates from `Decision::UpdateTray`.

### 2.6 UpdateChecker

- At launch and every 24 h: `GET https://api.github.com/repos/<owner>/moovy/releases/latest`.
- If `tag_name` is newer than the app version (semver), show "Update available: vX.Y.Z" in the tray and the settings window, linking to the release page.
- No download and no install. Failures are silent.
- Controlled by a build flag `MOOVY_CHANNEL = github | store`. In `store` builds the checker is compiled out (the Store updates the app).

### 2.7 Autostart and single instance

- `tauri-plugin-autostart` (writes the per-user Run entry). Enabled from onboarding when the checkbox is checked.
- `tauri-plugin-single-instance`: launching Moovy again opens the settings window instead of starting a second copy.

## 3. Frontend (React + TypeScript)

Two windows, one bundle, routed by window label.

### 3.1 Card window states

```text
           ShowCard(Break)                         ShowCard(Water)
                │                                         │
                ▼                                         ▼
          [break] ──3s──▶ [movement] ──Start──▶ [running] ──end──▶ [celebration] ──4s──▶ hide
                              │  │                  │ (Switch sides at 50% when two-sided)
                         Snooze  X                  X
                              │  │
                              ▼  ▼
                         [waiting]──2s──▶ hide        [water] ──Done──▶ bounce ──▶ hide
                                                         │ Snooze → [waiting], X → hide
```

- The card reports every user action back to Rust (`invoke("card_action", ...)`). Rust stays the source of truth.
- The timer runs in the card, and Rust receives `timer_finished`.

### 3.2 Motion

- CSS only: breathing (scale 1 → 1.02, 4 s loop), bounce on celebration and water Done, sway on waiting.
- Turned off when `prefers-reduced-motion: reduce`, which follows the Windows "Animation effects" setting.

### 3.3 Content

`src/content/movements.json` holds the six movements from PRD section 8: `id`, `category`, `name`, `cardLine`, `howTo`, `alternative`, `seconds`, `twoSided`, `reps`.

## 4. Character packs

```text
assets/characters/<id>/
  character.json
  idle.png  break.png  celebration.png  water.png  waiting.png
  wall-angels.png  spine-twist.png  hip-flexor.png
  neck-flexion.png  upper-trap.png  doorway-chest.png
```

```json
{
  "schemaVersion": 1,
  "id": "<character-id>",
  "name": "<Character Name>",
  "author": "Ori Eivan",
  "license": "<art license>",
  "states": {
    "idle": "idle.png", "break": "break.png", "celebration": "celebration.png",
    "water": "water.png", "waiting": "waiting.png"
  },
  "poses": {
    "wall-angels": "wall-angels.png", "spine-twist": "spine-twist.png", "hip-flexor": "hip-flexor.png",
    "neck-flexion": "neck-flexion.png", "upper-trap": "upper-trap.png", "doorway-chest": "doorway-chest.png"
  }
}
```

**Fallback:** if a pose file is missing, the card shows the `break` image for that movement. If a state file is missing, it shows `idle`. A missing image never breaks a card.

v0.1 bundles one pack and has no pack picker. The picker arrives with the second character.

## 5. Repository layout

```text
moovy/
├── README.md            logo, demo GIF, badges, download, SmartScreen guide, FAQ
├── LICENSE              code license
├── ASSETS_LICENSE       character art license
├── PRIVACY.md  CONTRIBUTING.md  SECURITY.md  CHANGELOG.md
├── docs/                PRD.md  ARCHITECTURE.md  TASKS.md  CHARACTER_PROMPT_KIT.md
├── assets/characters/<id>/
├── src/                 React: card/, settings/, content/movements.json
├── src-tauri/
│   ├── src/
│   │   ├── scheduler/   pure logic + tests
│   │   ├── activity/    Windows signals
│   │   ├── card.rs  tray.rs  store.rs  update.rs  lib.rs  main.rs
│   └── tauri.conf.json
└── .github/workflows/   ci.yml (tests on every push), release.yml (installer on tag)
```

## 6. Build and release

- **Installer:** Tauri NSIS bundle, per-user install mode (no admin prompt). **[verify]** the current `bundle.windows.nsis.installMode` option name.
- **WebView2:** the default bootstrapper is fine for GitHub builds. The Store build in v0.2 needs the offline installer mode.
- **CI:** `cargo test` (scheduler), `cargo clippy`, TypeScript typecheck, on every push.
- **Release:** pushing a tag `v0.1.0` builds the installer on a Windows runner and attaches it to a GitHub Release.
- **Signing:** none in v0.1. README shows the "More info → Run anyway" path with a screenshot.

## 7. v0.2 path (designed for, not built)

- **Microsoft Store:** MSIX packaging step around the Tauri build, `runFullTrust`, `StartupTask` for autostart, `MOOVY_CHANNEL=store`. The step-by-step Store plan is kept outside the repo, in the project's planning folder.
- **More characters:** a pack picker in settings, with the same contract.
- **Hardware companion:** the Scheduler's `Decision` stream is the natural thing to mirror to a device later. Nothing is built for it now.

## 8. Testing

| Layer | How |
|---|---|
| Scheduler | Unit tests with a simulated clock: idle pause, 15 min reset, sleep gap, min gap, snooze push, queue max 1, drop after 20 min, busy and mic suppression, pause and resume, water off, rotation |
| Activity | Manual checklist on real Windows (TASKS.md, Dogfood gate) |
| Card | Manual: no focus steal while typing, position on every monitor and taskbar side, 100% / 125% / 150% / 200% scaling |
| Performance | 8 hour run, Task Manager readings recorded in CHANGELOG |
