# Privacy

Moovy runs entirely on your computer. It has no account, no analytics, no telemetry, and it makes **no network requests**. Nothing about you or your computer leaves your machine.

_Last updated: 19 October 2026 (v0.1.0)_

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

One file on your computer: `%APPDATA%\com.orieivan.moovy\moovy.json`. It holds:

- Your settings: water reminders on or off, your character, whether setup is done.
- Where you dragged the card, and which movement comes next.
- Daily counts of what happened to the cards: shown, started, done, snoozed, skipped. Only numbers, kept on your computer. **Copy my stats** in the tray copies a short summary to your clipboard, and only when you click it. Sharing it is up to you.

**Start with Windows** adds a Moovy entry to your Windows startup list. You can turn it off in the settings window.

## Deleting your data

- **Settings → Clear my data** forgets the card position, the movement order and the daily counts. Your settings stay.
- Uninstalling Moovy (Windows Settings → Apps → Moovy → Uninstall) removes the app and its startup entry.
- To remove everything, delete the folder `%APPDATA%\com.orieivan.moovy` after uninstalling.

## Links

The settings window links to GitHub (the project and "Report a problem"). These open in your browser only when you click them.

## Changes

If a future version ever needs the network (for example to check for updates), this page will say exactly what it sends, before that version is released.

Questions: open an issue at https://github.com/ori-eivan/moovy/issues
