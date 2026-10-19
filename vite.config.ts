import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
// @ts-expect-error type error without @types/node package
import process from "node:process";
// @ts-expect-error type error without @types/node package
import { readdirSync, rmSync } from "node:fs";
const host = process.env.TAURI_DEV_HOST;

/** Character packs that may ship. Any other folder in public/characters/ is local only
 *  (third-party art for testing) and is removed from every build, so it never reaches an
 *  installer. */
const SHIPPED_CHARACTERS = ["thomas", "corni"];

const shipOnlyOwnCharacters = {
  name: "ship-only-own-characters",
  apply: "build" as const,
  closeBundle() {
    const dir = "dist/characters";
    for (const name of readdirSync(dir) as string[]) {
      if (!SHIPPED_CHARACTERS.includes(name)) {
        rmSync(`${dir}/${name}`, { recursive: true, force: true });
      }
    }
  },
};

// https://vite.dev/config/
export default defineConfig(() => ({
  plugins: [react(), shipOnlyOwnCharacters],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
