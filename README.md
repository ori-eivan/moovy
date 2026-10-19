# Moovy

A small, friendly companion for Windows that reminds you to drink water and move while you work at your computer.

> **Status: v0.1, early testing.** Windows 10 and 11.

## What it does

Every 30 minutes of active computer time, a small card pops up at the bottom-left of your screen, alternating between a glass of water and a 30-second movement or stretch. One click to start, snooze or skip. Then it gets out of your way.

- Never takes your keyboard focus
- Waits while you're in a call (even muted), sharing your screen, presenting or in a full-screen app, and is left out of screen shares and recordings
- No account, no network, no tracking. Everything stays on your computer ([Privacy](PRIVACY.md))
- English only

## Install

1. Download `Moovy_0.1.0_x64-setup.exe` from the [Releases page](https://github.com/ori-eivan/moovy/releases).
2. Run it. It installs for your user only, so it doesn't ask for administrator rights.
3. **Windows may show "Windows protected your PC"** (SmartScreen). This appears for new apps that are not code-signed yet, not because something is wrong. Click **More info**, then **Run anyway**. The publisher shows as "Unknown publisher" for the same reason.
4. Moovy opens its settings window once, then lives in the tray (the `^` arrow next to the clock) and starts with Windows. The first card comes after 30 minutes of active work.

**Uninstall:** Windows Settings → Apps → Installed apps → Moovy → Uninstall.

### Install from the command line

Handy if an assistant or a script sets it up for you. The installer is per-user, needs no administrator rights, and Moovy makes no network requests ([Privacy](PRIVACY.md)).

```powershell
$setup = "$env:TEMP\Moovy_0.1.0_x64-setup.exe"
Invoke-WebRequest "https://github.com/ori-eivan/moovy/releases/download/v0.1.0/Moovy_0.1.0_x64-setup.exe" -OutFile $setup
(Get-FileHash $setup -Algorithm SHA256).Hash   # should be 28123F94EF55EA6002FEAD2A1AD1E1C0C8151583BE8072F911711E6B46529A54
Start-Process $setup -ArgumentList "/S" -Wait   # silent, current user only
Start-Process "$env:LOCALAPPDATA\Moovy\moovy.exe"
```

Moovy then sits in the tray and starts with Windows. The first card appears after 30 minutes of active work.

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

Moovy's default character is **Thomas**, a tabby cat, in `public/characters/thomas/`. Each character is a folder of 11 images (one per card state and movement). Other folders in `public/characters/` are local only and never committed. The settings window lists only the characters whose images are present.

## License

Code: [MIT](LICENSE). Character art (Thomas): [CC BY-NC 4.0](ASSETS_LICENSE), not covered by the MIT license.

Made by Ori Eivan.
