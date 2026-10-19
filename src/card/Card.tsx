import { useEffect, useLayoutEffect, useRef, useState, type MouseEvent } from "react";
import { DEFAULT_CHARACTER } from "../content/characters";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import CardView from "./CardView";
import { useCardFlow, type Payload } from "./useCardFlow";

/** Room around the card for its shadow, in px. Must match the card's offset in card.css. */
const SHADOW_ROOM = 20;
/** Extra room above the hello card for its speech bubble. Matches .bubble in card.css. */
const BUBBLE_ROOM = 64;

const appWindow = getCurrentWebviewWindow();

/** The real card window: the shared flow, wired to Rust. */
export default function Card() {
  const cardRef = useRef<HTMLDivElement>(null);
  const { view, now, show, actions } = useCardFlow({
    started: () => invoke("card_started"),
    done: (kind) => invoke("card_done", { kind }),
    skip: () => invoke("card_skip"),
    snooze: () => invoke("card_snooze"),
    helloDone: () => invoke("card_hello_done"),
  });
  const viewRef = useRef(view);
  viewRef.current = view;
  const actionsRef = useRef(actions);
  actionsRef.current = actions;

  const [character, setCharacter] = useState(DEFAULT_CHARACTER);

  useEffect(() => {
    document.body.classList.add("card-window");
    const unlisten = appWindow.listen<Payload>("card:show", ({ payload }) => {
      // The character can change in settings between two cards.
      invoke<{ character: string }>("get_settings").then((s) => setCharacter(s.character));
      show(payload);
    });
    // The hello card: Windows tells Rust when the user lets go of the drag.
    const unlistenDrag = appWindow.listen("card:drag-end", () => {
      const v = viewRef.current;
      if (v.name === "hello" && !v.saved) actionsRef.current.helloSaved();
    });
    return () => {
      unlisten.then((stop) => stop());
      unlistenDrag.then((stop) => stop());
    };
  }, [show]);

  // The window always fits the card, so no invisible area blocks clicks on the desktop.
  useLayoutEffect(() => {
    const el = cardRef.current;
    if (!el) return;
    const extra = view.name === "hello" ? BUBBLE_ROOM : 0;
    const report = () => invoke("card_resize", { height: el.offsetHeight + SHADOW_ROOM + extra });
    report();
    const observer = new ResizeObserver(report);
    observer.observe(el);
    return () => observer.disconnect();
  }, [view.name]);

  // Drag the card anywhere. Buttons keep working normally.
  const onMouseDown = (event: MouseEvent) => {
    if (event.button !== 0) return;
    if ((event.target as HTMLElement).closest("button")) return;
    invoke("card_drag_start");
    appWindow.startDragging();
  };

  return <CardView ref={cardRef} view={view} now={now} actions={actions} character={character} onMouseDown={onMouseDown} />;
}
