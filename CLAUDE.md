# CLAUDE.md: Moovy

Moovy is a desktop tray app for **Windows and macOS** (Tauri v2: Rust in `src-tauri/`, React + TypeScript in `src/`). Product: `docs/PRD.md`. Architecture: `docs/ARCHITECTURE.md`.

Before any change to how Moovy looks or moves, read `docs/DESIGN_SYSTEM.md`.

Open tasks and their order (updates, licence, publishing): `../docs/NEXT_TASKS.md`, outside this repository. Read it before starting on any of them.

## One folder, branches inside it

- All Moovy work happens in `app/`. A new branch is made in this folder (`git switch -c <name>`), and you move between branches with `git switch`. No extra folders: Ori does not want copies of the project piling up (decided 25.10.2026, after `app-last-things` and `app-release` took 6 GB).
- Before switching branch, check `git status`. Unfinished changes are committed first, never left to travel to the other branch.
- Commit only the files you changed yourself, by name. Never `git add -A` or `git add .` here.
- A second folder (`git worktree`) only when another session is working on Moovy right now with unfinished changes, and only after asking Ori. Remove it with `git worktree remove` as soon as its branch is merged.
- Build output (`src-tauri/target`) grows by gigabytes per release. Old installers are on GitHub Releases, so `cargo clean` is always safe.

## Every new version goes through the same four steps

1. A new branch for the version. Never bump the version on `main`.
2. Ori tests the build locally, on his own Windows.
3. The GitHub checks run on the branch and pass.
4. Only then is it merged. Ori merges, after both 2 and 3 passed.

**Install and launch the test build outside Claude.** The Claude desktop app is a packaged app: anything a session installs or runs inherits its sandbox, so files under `AppData` and keys under `HKCU` land in `C:\Users\user\AppData\Local\Packages\Claude_*\LocalCache` instead of the real locations. The install looks fine from inside Claude and does not exist for Windows: no Start with Windows, broken shortcuts, and the app dies when Claude restarts (seen 22.10.2026 with 0.1.8). Run the installer and the app through a process outside the package, for example `Invoke-CimMethod -ClassName Win32_Process -MethodName Create -Arguments @{CommandLine='"<path>\Moovy_X.Y.Z_x64-setup.exe"'}`, or ask Ori to double-click it. Launch only programs with their own window this way: a `cmd`, `reg` or `tasklist` started like this opens a terminal window on Ori's screen. To check what Windows really sees, read it through WMI, which starts no process: `StdRegProv` on `HKEY_USERS\<SID>\Software\Microsoft\Windows\CurrentVersion\Run` for Start with Windows, `CIM_DataFile` for `%LOCALAPPDATA%\Moovy\moovy.exe`, `Win32_Process` for the running copy.

## Every change ships on both systems

- A feature is done only when it works on Windows **and** on macOS. Never build for one and leave the other behind.
- System-specific code lives behind `cfg` in two places: `src-tauri/src/activity.rs` (signals: idle, lock, mic, sharing) and the `platform` module in `src-tauri/src/card.rs` (`win32` for Windows, `portable` for macOS). New system-specific code goes there, with a version for each system.
- Words that differ per system (e.g. "Start with Windows" / "Start at login", tray / menu bar) are chosen at runtime, never hard-coded for one.
- Check Windows locally (`cargo clippy --all-targets`, `cargo test --lib`, `npm run tauri build`). Check macOS on GitHub: the `macOS build` workflow builds, runs the tests and screenshots the first launch on a hosted Mac.
- A release carries both files twice: with the version in the name (`Moovy_X.Y.Z_x64-setup.exe`, `Moovy_X.Y.Z_universal.dmg`) and with a fixed name (`Moovy-Windows-setup.exe`, `Moovy-Mac.dmg`). The fixed names keep the README and shared links (`/releases/latest/download/...`) working for every new version. Update the README checksums in the same release.
- Every release adds one row at the top of `CHANGELOG.md` (version, date, systems, what changed in plain words), in the same commit as the version bump. A version without a row is not released. Ori tracks the project from this table.
- Since 0.1.7 a release also carries the update files: `Moovy_X.Y.Z_x64-setup.exe.sig`, `Moovy_X.Y.Z_universal.app.tar.gz` and its `.sig` (renamed from the CI artifact `Moovy.app.tar.gz`), and `latest.json` from `node scripts/updater-manifest.mjs`. Without `latest.json` on the newest release, nobody gets the update.
- Updates are signed. Build Windows with `TAURI_SIGNING_PRIVATE_KEY` set to `C:\Users\user\shared_api\moovy-updater.key` and `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` read from `MOOVY_UPDATER_KEY_PASSWORD` in `C:\Users\user\shared_api\.env.local` (never print it). The Mac build reads the same pair from GitHub secrets. If the key is lost, installed copies can never be updated again.
