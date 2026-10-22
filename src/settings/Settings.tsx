import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";
import Onboarding from "./Onboarding";
import SettingsView, { type SettingsState } from "./SettingsView";

const REPO = "https://github.com/ori-eivan/moovy";

/** The real settings window: the view, wired to Rust. */
export default function Settings() {
  const [state, setState] = useState<SettingsState | null>(null);

  useEffect(() => {
    invoke<SettingsState>("get_settings").then(setState);
  }, []);

  if (!state) return null;

  if (!state.onboarded) {
    return (
      <Onboarding
        onFinish={({ character, pace, updates }) => {
          // Rust saves the choices, hides this window and shows the hello card.
          invoke("finish_onboarding", {
            character,
            pace: pace.id,
            waterMinutes: pace.waterMinutes ?? 0,
            moveMinutes: pace.moveMinutes,
            updates,
          }).then(() =>
            setState({ ...state, character, pace, updates, onboarded: true }),
          );
        }}
      />
    );
  }

  return (
    <SettingsView
      state={state}
      actions={{
        setCharacter: (id) => {
          setState({ ...state, character: id });
          invoke("set_character", { id });
        },
        setPace: (pace) => {
          setState({ ...state, pace });
          invoke("set_pace", { id: pace.id, waterMinutes: pace.waterMinutes ?? 0, moveMinutes: pace.moveMinutes });
        },
        setAutostart: (enabled) => {
          setState({ ...state, autostart: enabled });
          invoke("set_autostart", { enabled }).catch(() => setState({ ...state, autostart: !enabled }));
        },
        setUpdates: (enabled) => {
          setState({ ...state, updates: enabled });
          invoke("set_updates", { enabled });
        },
        clearData: () => invoke("clear_data"),
        openGitHub: () => openUrl(REPO),
        reportProblem: () =>
          openUrl(`${REPO}/issues/new?title=&body=${encodeURIComponent(`Moovy version: ${state.version}\n\nWhat happened:\n`)}`),
      }}
    />
  );
}
