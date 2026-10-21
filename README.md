# Moovy

A small, friendly companion for Windows (and Mac, in beta) that reminds you to drink water and move while you work at your computer.

> **Status: v0.1, early testing.** Windows 10 and 11. Mac (macOS 11 or later) in beta.

## Download

**You don't need the source code to use Moovy.** Download the app for your system:

- **Windows:** [Moovy-Windows-setup.exe](https://github.com/ori-eivan/moovy/releases/latest/download/Moovy-Windows-setup.exe)
- **Mac (beta):** [Moovy-Mac.dmg](https://github.com/ori-eivan/moovy/releases/latest/download/Moovy-Mac.dmg)

These links always point to the latest version. Install steps, the first-run security prompt, and checksums are below. Cloning this repository is only needed to work on Moovy itself.

## What it does

A small card pops up at the bottom-left of your screen during active computer time: a glass of water, or a 30-second movement or stretch. You pick how often, from a preset or your own minutes. One click to start, snooze or skip. Then it gets out of your way.

- Never takes your keyboard focus
- Waits while you're in a call (even muted), sharing your screen, presenting or in a full-screen app, and is left out of screen shares and recordings
- No account, no network, no tracking. Everything stays on your computer ([Privacy](PRIVACY.md))
- English only

## Install on Windows

1. Download [Moovy-Windows-setup.exe](https://github.com/ori-eivan/moovy/releases/latest/download/Moovy-Windows-setup.exe) (the same file as `Moovy_0.1.6_x64-setup.exe` on the [Releases page](https://github.com/ori-eivan/moovy/releases)).
2. **Your browser may hold the download** with a note like "isn't commonly downloaded" (Chrome) or "isn't commonly downloaded / could harm your device" (Edge). This happens to new apps that aren't code-signed yet. Open the downloads list, click **⋯** next to the file, then **Keep** (in Edge: **Keep**, then **Show more → Keep anyway**).
3. Run it. It installs for your user only, so it doesn't ask for administrator rights.
4. **Windows may show "Windows protected your PC"** (SmartScreen). This appears for new apps that are not code-signed yet, not because something is wrong. Click **More info**, then **Run anyway**. The publisher shows as "Unknown publisher" for the same reason.
5. A short setup opens: pick your buddy and how often you want reminders. Then Moovy lives in the tray (the `^` arrow next to the clock) and starts with Windows. The first card comes after 45 minutes of active work (on the Recommended pace).

**Uninstall:** Windows Settings → Apps → Installed apps → Moovy → Uninstall.

### Install from the command line

Handy if an assistant or a script sets it up for you. The installer is per-user, needs no administrator rights, and Moovy makes no network requests ([Privacy](PRIVACY.md)).

```powershell
$setup = "$env:TEMP\Moovy_0.1.6_x64-setup.exe"
Invoke-WebRequest "https://github.com/ori-eivan/moovy/releases/download/v0.1.6/Moovy_0.1.6_x64-setup.exe" -OutFile $setup
(Get-FileHash $setup -Algorithm SHA256).Hash   # should be F3442119A23B3E15DBBAD7336BCFE1667CF59A70D9A46472A7F609FE50C173ED
Start-Process $setup -ArgumentList "/S" -Wait   # silent, current user only
Start-Process "$env:LOCALAPPDATA\Moovy\moovy.exe"
```

Moovy then sits in the tray and starts with Windows. The first card appears after 45 minutes of active work (on the Recommended pace).

## Install on Mac (beta)

The Mac version is new and has had little real-world testing yet. Feedback is very welcome: [docs/MAC_BETA_TEST.md](docs/MAC_BETA_TEST.md) lists five quick checks.

1. Download [Moovy-Mac.dmg](https://github.com/ori-eivan/moovy/releases/latest/download/Moovy-Mac.dmg) (the same file as `Moovy_0.1.6_universal.dmg`, one file for Apple silicon and Intel).
2. Open it and drag **Moovy** into **Applications**.
3. Open Moovy. **macOS will say it can't check the app**, because Moovy is not notarized by Apple yet. Open **System Settings → Privacy & Security**, scroll down, and click **Open Anyway**. Only the first time.
4. The same short setup opens. Moovy then lives in the menu bar (top right), with no Dock icon.

Moovy asks for no permissions on Mac. One difference from Windows: it doesn't recognise screen sharing yet (that would need the Screen Recording permission). It still waits during calls, and the card is left out of screen shares and recordings.

**Uninstall:** in Moovy's settings turn off **Start at login**, quit Moovy from its menu, then drag it from Applications to the Bin.

From the command line (a file downloaded this way is not flagged by Gatekeeper):

```bash
curl -L -o /tmp/Moovy.dmg https://github.com/ori-eivan/moovy/releases/download/v0.1.6/Moovy_0.1.6_universal.dmg
shasum -a 256 /tmp/Moovy.dmg   # should be 8ca79c65d24e9f85485246e84d77e258abbb03873b39a06f2c442d0a93928c70
hdiutil attach -nobrowse -quiet /tmp/Moovy.dmg -mountpoint /tmp/moovy-dmg
cp -R /tmp/moovy-dmg/Moovy.app /Applications/
hdiutil detach -quiet /tmp/moovy-dmg
open /Applications/Moovy.app
```

The installer is the recommended way to use Moovy: it is the only version that includes the full design, with every font. A copy built from this source code works the same but shows a fallback font for headings and buttons (see [Fonts](#fonts)).

## Development

Requires Node.js, Rust and the Microsoft C++ Build Tools ([Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)).

```bash
npm install
npm run tauri dev
```

- `http://localhost:1420/gallery.html` shows every card and settings screen in the browser while the dev server runs.
- Rust tests: `cargo test` inside `src-tauri/`.
- Product and technical docs live in [`docs/`](docs/).

### Fonts

- Labels and numbers: [Hyperlegible Sans](https://github.com/matthewlarn/hyperlegible-sans), SIL Open Font License, bundled.
- Long descriptions: [Manrope](https://github.com/sharanda/manrope), SIL Open Font License, bundled.
- Headings and buttons: [Clash Display](https://www.fontshare.com/fonts/clash-display) and [Chillax](https://www.fontshare.com/fonts/chillax) by Indian Type Foundry. Their license does not allow redistributing the font files through a repository, so they are **not** included. Download them from Fontshare and place `ClashDisplay-Variable.woff2` in `public/fonts/clash-display/` and `Chillax-Variable.woff2` in `public/fonts/chillax/`. Without them, the UI falls back to Hyperlegible Sans. The installer on the Releases page already includes them, embedded in the app as the license allows.

### Character art

Moovy ships two characters: **Thomas**, a tabby cat (the default), in `public/characters/thomas/`, and **Corni**, a cornichon pickle, in `public/characters/corni/`. Each character is a folder of 11 images (one per card state and movement). Other folders in `public/characters/` are local only and never committed. The settings window lists only the characters whose images are present.

### Icons

The 💧 and 🚶 icons are from [Microsoft Fluent Emoji](https://github.com/microsoft/fluentui-emoji) (Flat style), MIT licence: [`public/icons/FLUENT-EMOJI-LICENSE.txt`](public/icons/FLUENT-EMOJI-LICENSE.txt).

## Free to use, not for sale

Moovy is free to use, at home or at work. The code is open to read, learn from and change for yourself. Selling Moovy, or building a paid product from its code, is not allowed without permission from Ori Eivan.

The terms: code under the [PolyForm Noncommercial License 1.0.0](LICENSE), with an extra permission to use the official app at work. Character art (Thomas, Corni): [CC BY-NC 4.0](ASSETS_LICENSE). Versions 0.1.5 and earlier stay under MIT.

Made by Ori Eivan.
