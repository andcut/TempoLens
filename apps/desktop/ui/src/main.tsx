import React from "react";
import ReactDOM from "react-dom/client";
import { AnalyzePage } from "./pages/AnalyzePage";
import { ThemeToggle } from "./components/ThemeToggle";
import "./styles/app.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ThemeToggle />
    <AnalyzePage />
  </React.StrictMode>
);
