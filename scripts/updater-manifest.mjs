// Writes latest.json, the file Moovy's update check reads (src-tauri/src/updates.rs).
// It goes on each GitHub release next to the installers, so the permanent link
// /releases/latest/download/latest.json always describes the newest version.
//
// Usage: node scripts/updater-manifest.mjs <version> <windows .exe.sig> <mac .app.tar.gz.sig> [out]

import { readFileSync, writeFileSync } from "node:fs";

const [version, windowsSig, macSig, out = "latest.json"] = process.argv.slice(2);
if (!version || !windowsSig || !macSig) {
  console.error("Usage: node scripts/updater-manifest.mjs <version> <windows .exe.sig> <mac .app.tar.gz.sig> [out]");
  process.exit(1);
}

const base = `https://github.com/ori-eivan/moovy/releases/download/v${version}`;
const signature = (path) => readFileSync(path, "utf8").trim();
// One universal Mac file serves both Apple silicon and Intel.
const mac = { signature: signature(macSig), url: `${base}/Moovy_${version}_universal.app.tar.gz` };

const manifest = {
  version,
  notes: `Moovy ${version}. What's new: https://github.com/ori-eivan/moovy/releases/tag/v${version}`,
  pub_date: new Date().toISOString(),
  platforms: {
    "windows-x86_64": { signature: signature(windowsSig), url: `${base}/Moovy_${version}_x64-setup.exe` },
    "darwin-aarch64": mac,
    "darwin-x86_64": mac,
  },
};

writeFileSync(out, JSON.stringify(manifest, null, 2) + "\n");
console.log(`Wrote ${out} for ${version}`);
