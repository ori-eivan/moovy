# Privacy

Moovy runs entirely on your computer. It has no account, no analytics and no telemetry. Nothing about you or your computer leaves your machine.

Moovy talks to the internet for one thing only: once a day it asks GitHub whether there is a newer version. You can switch that off, and then Moovy makes no network requests at all.

_Last updated: 22 October 2026 (v0.1.7)_

## The one network request: checking for updates

- **What is sent:** a request for one public file, [`latest.json`](https://github.com/ori-eivan/moovy/releases/latest/download/latest.json), which holds the newest version number and its download links. Moovy adds nothing about you: no ID, no settings, no counts. Like any website, GitHub sees your IP address and that the request comes from Moovy.
- **How often:** at most once a day, starting a couple of minutes after Moovy opens, plus whenever you click **Check for updates** in the tray.
- **When there is a new version:** Moovy downloads it in the background and checks its digital signature, so only versions signed by Ori Eivan are accepted. Then your character asks: **Install** or **Later**. Nothing installs without your click.
- **Switching it off:** **Check for updates** in the settings window (and on the last setup screen). Off means no request is made at all, except when you click **Check for updates** in the tray yourself.
- **The code:** all of it is in [`src-tauri/src/updates.rs`](https://github.com/ori-eivan/moovy/blob/main/src-tauri/src/updates.rs).

If you installed Moovy before 0.1.7, nothing is checked until your character asks you once whether to keep update checks on.

## What Moovy reads, and why

To show a card at a good moment and never during a call, Moovy asks Windows a few questions once a second. The answers are used on the spot and are never stored or sent.

| What | Why | What Moovy does **not** do |
|---|---|---|
| Time since your last keyboard or mouse input | To count only active computer time and notice when you're away | It never sees which keys you press or where you click |
| Whether the screen is locked, or Windows reports busy (presentation, full-screen app, quiet time) | To hold cards until a good moment | |
| Whether any app is recording from a microphone | A call is in progress, so cards wait | It never opens, records or listens to the microphone |
| The titles of open windows | To notice the small "is sharing your screen" bar of Chrome, Edge or Zoom, so cards wait while you share | Titles are compared and discarded, never stored |

The card is also marked so that screen sharing, recordings and screenshots leave it out.

## What Moovy stores

One file on your computer: `%APPDATA%\com.orieivan.moovy\moovy.json` on Windows, `~/Library/Application Support/com.orieivan.moovy/moovy.json` on a Mac. It holds:

- Your settings: your pace, your character, update checks on or off, whether setup is done, and when updates were last checked.
- Where you dragged the card, and which movement comes next.
- Daily counts of what happened to the cards: shown, started, done, snoozed, skipped. Only numbers, kept on your computer.
- For the Stats tab, a list of the cards you finished or closed in the last 30 days: the time, water or which movement, and done or skipped. Kept on your computer only. **Clear my data** deletes it. **Copy my stats** in the tray copies a short summary to your clipboard, and only when you click it. Sharing it is up to you.

**Start with Windows** adds a Moovy entry to your Windows startup list. You can turn it off in the settings window.

## Deleting your data

- **Settings → Clear my data** forgets the card position, the movement order and the daily counts. Your settings stay.
- Uninstalling Moovy (Windows Settings → Apps → Moovy → Uninstall) removes the app and its startup entry.
- To remove everything, delete the folder `%APPDATA%\com.orieivan.moovy` after uninstalling.

## Links

The settings window links to GitHub (the project and "Report a problem"). These open in your browser only when you click them.

## Changes

If a future version ever sends anything more, this page will say exactly what, before that version is released.

Questions: open an issue at https://github.com/ori-eivan/moovy/issues
