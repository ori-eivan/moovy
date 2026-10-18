import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import Card from "./card/Card";
import Settings from "./settings/Settings";

// One bundle, two windows. The window label decides what renders.
const label = getCurrentWebviewWindow().label;

export default function App() {
  return label === "card" ? <Card /> : <Settings />;
}
