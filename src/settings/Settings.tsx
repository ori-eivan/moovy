import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";
import SettingsView, { type SettingsState } from "./SettingsView";

const REPO = "https://github.com/ori-eivan/moovy";

/** The real settings window: the view, wired to Rust. */
export default function Settings() {
  const [state, setState] = useState<SettingsState | null>(null);

  useEffect(() => {
    invoke<SettingsState>("get_settings").then(setState);
  }, []);

  if (!state) return null;

  return (
    <SettingsView
      state={state}
      actions={{
        setCharacter: (id) => {
          setState({ ...state, character: id });
          invoke("set_character", { id });
        },
        setWater: (enabled) => {
          setState({ ...state, waterEnabled: enabled });
          invoke("set_water_enabled", { enabled });
        },
        setAutostart: (enabled) => {
          setState({ ...state, autostart: enabled });
          invoke("set_autostart", { enabled }).catch(() => setState({ ...state, autostart: !enabled }));
        },
        clearData: () => invoke("clear_data"),
        openGitHub: () => openUrl(REPO),
        reportProblem: () =>
          openUrl(`${REPO}/issues/new?title=&body=${encodeURIComponent(`Moovy version: ${state.version}\n\nWhat happened:\n`)}`),
      }}
    />
  );
}
