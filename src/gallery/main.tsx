import React from "react";
import ReactDOM from "react-dom/client";
import Gallery from "./Gallery";

// Dev-only page (http://localhost:1420/gallery.html): every card screen in a browser,
// without Tauri. Used to review designs at real size.
ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Gallery />
  </React.StrictMode>,
);
