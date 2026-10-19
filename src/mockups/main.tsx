import React from "react";
import ReactDOM from "react-dom/client";
import OnboardingMockups from "./OnboardingMockups";

// Dev-only page (http://localhost:1420/onboarding-mockups.html): static onboarding mockups to
// review and annotate before anything is built. Not part of the app build.
ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <OnboardingMockups />
  </React.StrictMode>,
);
