# Moovy

A small, friendly companion for Windows that reminds you to drink water and move while you work at your computer.

> **Status: early development (v0.1 in progress).** Not ready to install yet. Nothing below is a finished feature until it is released.

## What it does

Every 30 minutes of active computer time, a small card pops up at the bottom-left of your screen, alternating between a glass of water and a one-minute movement or stretch. One click to start, snooze or skip. Then it gets out of your way.

- Never takes your keyboard focus, never shows during calls or presentations (planned)
- No account, no cloud, no tracking. Everything stays on your computer
- English only

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
- Headings and buttons: [Clash Display](https://www.fontshare.com/fonts/clash-display) and [Chillax](https://www.fontshare.com/fonts/chillax) by Indian Type Foundry. Their license does not allow redistributing the font files through a repository, so they are **not** included. Download them from Fontshare and place `ClashDisplay-Variable.woff2` in `public/fonts/clash-display/` and `Chillax-Variable.woff2` in `public/fonts/chillax/`. Without them, the UI falls back to Hyperlegible Sans.

### Character art

Character images live in `public/characters/<id>/` and are not in the repository yet. Without them, cards show the name of the missing image file.

## License

Code: [MIT](LICENSE). Character art: CC BY-NC 4.0 (added with the art).

Made by Ori Eivan.
