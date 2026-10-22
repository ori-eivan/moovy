# CLAUDE.md: Moovy

Moovy is a desktop tray app for **Windows and macOS** (Tauri v2: Rust in `src-tauri/`, React + TypeScript in `src/`). Product: `docs/PRD.md`. Architecture: `docs/ARCHITECTURE.md`.

Before any change to how Moovy looks or moves, read `docs/DESIGN_SYSTEM.md`.

Open tasks and their order (updates, licence, publishing): `../docs/NEXT_TASKS.md`, outside this repository. Read it before starting on any of them.

## Several sessions work on Moovy at once

- `app/` may be in use by another session, on its own branch, with unfinished changes. Before any commit, check the branch (`git branch --show-current`) and `git status`.
- Commit only the files you changed yourself, by name. Never `git add -A` or `git add .` here.
- Releases and work that must not mix with someone else's go in a separate git worktree (for example `../app-release`), never in a folder another session is using.

## Every change ships on both systems

- A feature is done only when it works on Windows **and** on macOS. Never build for one and leave the other behind.
- System-specific code lives behind `cfg` in two places: `src-tauri/src/activity.rs` (signals: idle, lock, mic, sharing) and the `platform` module in `src-tauri/src/card.rs` (`win32` for Windows, `portable` for macOS). New system-specific code goes there, with a version for each system.
- Words that differ per system (e.g. "Start with Windows" / "Start at login", tray / menu bar) are chosen at runtime, never hard-coded for one.
- Check Windows locally (`cargo clippy --all-targets`, `cargo test --lib`, `npm run tauri build`). Check macOS on GitHub: the `macOS build` workflow builds, runs the tests and screenshots the first launch on a hosted Mac.
- A release carries both files twice: with the version in the name (`Moovy_X.Y.Z_x64-setup.exe`, `Moovy_X.Y.Z_universal.dmg`) and with a fixed name (`Moovy-Windows-setup.exe`, `Moovy-Mac.dmg`). The fixed names keep the README and shared links (`/releases/latest/download/...`) working for every new version. Update the README checksums in the same release.
- Every release adds one row at the top of `CHANGELOG.md` (version, date, systems, what changed in plain words), in the same commit as the version bump. A version without a row is not released. Ori tracks the project from this table.
- Since 0.1.7 a release also carries the update files: `Moovy_X.Y.Z_x64-setup.exe.sig`, `Moovy_X.Y.Z_universal.app.tar.gz` and its `.sig` (renamed from the CI artifact `Moovy.app.tar.gz`), and `latest.json` from `node scripts/updater-manifest.mjs`. Without `latest.json` on the newest release, nobody gets the update.
- Updates are signed. Build Windows with `TAURI_SIGNING_PRIVATE_KEY` set to `C:\Users\user\shared_api\moovy-updater.key` and `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` read from `MOOVY_UPDATER_KEY_PASSWORD` in `C:\Users\user\shared_api\.env.local` (never print it). The Mac build reads the same pair from GitHub secrets. If the key is lost, installed copies can never be updated again.
