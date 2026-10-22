# Moovy on Mac: beta test (about 10 minutes)

Thanks for trying Moovy on a Mac. It's the first time it runs on one, so a few things may be rough. Each check below says what should happen; if it doesn't, a screenshot and one line is perfect.

## Install

Ask Claude: "Install Moovy for Mac from https://github.com/ori-eivan/moovy" (the README has a command-line section it can follow).

Or by hand: download the `.dmg` from the Releases page, drag Moovy to Applications, open it. macOS will say Apple can't check it (Moovy isn't notarized yet): open **System Settings → Privacy & Security**, scroll down, click **Open Anyway**. Only the first time.

## Checks

1. **First launch.** A small window asks you to pick a buddy, then a pace. A tap moves on by itself. After "Close", a card appears near the bottom-left corner with a bubble saying you can drag it. Drag it somewhere else and let go: the bubble should say "Saved!" and the card closes.
   - Expected: no Dock icon; Moovy lives in the menu bar (top right).
2. **The card doesn't steal your typing.** The first card comes after 45 minutes of work (Recommended pace). When it appears while you're typing in another app, your typing should continue in that app, and one click on the card's buttons should work.
3. **The card shows where you parked it** (where you dropped the hello card in check 1), on the screen you're working on.
4. **Calls.** Join a Zoom, Meet or FaceTime call with the microphone on. No card should appear during the call; a card that was due shows up after you hang up.
5. **Away and back.** Lock the screen (Control-Command-Q) for a minute, unlock: no card in the first 2 minutes.

## Send back

- Which checks passed, which didn't.
- Your macOS version (Apple menu → About This Mac) and whether it's Apple silicon (M1/M2/M3/M4) or Intel.
- From the menu bar icon: **Stats**, and a screenshot of that window.
