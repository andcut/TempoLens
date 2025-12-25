import type { GameAnalysis } from "./types";
import { MOCK_OPERA_HOUSE_ANALYSIS } from "./mockData";

// Check if we're running inside Tauri
function isTauriAvailable(): boolean {
  return typeof window !== "undefined" && "__TAURI_IPC__" in window;
}

export async function analyzePgnText(pgn: string, enginePath: string): Promise<GameAnalysis> {
  if (!isTauriAvailable()) {
    console.log("[TempoLens] Tauri not available, using mock Opera House Game analysis");
    // Simulate a short delay like a real analysis would have
    await new Promise((resolve) => setTimeout(resolve, 500));
    return MOCK_OPERA_HOUSE_ANALYSIS;
  }

  const { invoke } = await import("@tauri-apps/api/tauri");
  const raw = await invoke<string>("analyze_pgn_text", { pgn, enginePath });
  return JSON.parse(raw);
}

export async function analyzePgnFile(path: string, enginePath: string): Promise<GameAnalysis> {
  if (!isTauriAvailable()) {
    console.log("[TempoLens] Tauri not available, using mock Opera House Game analysis");
    await new Promise((resolve) => setTimeout(resolve, 500));
    return MOCK_OPERA_HOUSE_ANALYSIS;
  }

  const { invoke } = await import("@tauri-apps/api/tauri");
  const raw = await invoke<string>("analyze_pgn_file", { path, enginePath });
  return JSON.parse(raw);
}

