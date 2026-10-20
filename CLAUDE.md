# CLAUDE.md: Moovy

Moovy is a desktop tray app for **Windows and macOS** (Tauri v2: Rust in `src-tauri/`, React + TypeScript in `src/`). Product: `docs/PRD.md`. Architecture: `docs/ARCHITECTURE.md`.

Before any change to how Moovy looks or moves, read `docs/DESIGN_SYSTEM.md`.

## Every change ships on both systems

- A feature is done only when it works on Windows **and** on macOS. Never build for one and leave the other behind.
- System-specific code lives behind `cfg` in two places: `src-tauri/src/activity.rs` (signals: idle, lock, mic, sharing) and the `platform` module in `src-tauri/src/card.rs` (`win32` for Windows, `portable` for macOS). New system-specific code goes there, with a version for each system.
- Words that differ per system (e.g. "Start with Windows" / "Start at login", tray / menu bar) are chosen at runtime, never hard-coded for one.
- Check Windows locally (`cargo clippy --all-targets`, `cargo test --lib`, `npm run tauri build`). Check macOS on GitHub: the `macOS build` workflow builds, runs the tests and screenshots the first launch on a hosted Mac.
- A release carries both files: the Windows installer and the Mac `.dmg`.
