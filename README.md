# Moovy

A small, friendly companion for Windows that reminds you to drink water and move while you work at your computer.

> **Status: early development (v0.1 in progress).** Not ready to install yet. Nothing below is a finished feature until it is released.

## What it does

Every 30 minutes of active computer time, a small card pops up at the bottom-left of your screen, alternating between a glass of water and a one-minute movement or stretch. One click to start, snooze or skip. Then it gets out of your way.

- Never takes your keyboard focus, never shows during calls or presentations (planned)
- No account, no cloud, no tracking. Everything stays on your computer
- English only

## Download

**Get the installer from the [Releases page](https://github.com/ori-eivan/moovy/releases)** (available once v0.1 is released). The installer is the recommended way to use Moovy: it is the only version that includes the full design, with every font. A copy built from this source code works the same but shows a fallback font for headings and buttons (see [Fonts](#fonts)).

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
