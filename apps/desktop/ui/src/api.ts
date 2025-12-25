import type { GameAnalysis } from "./types";
import { MOCK_OPERA_HOUSE_ANALYSIS } from "./mockData";

// Check if we're running inside Tauri
function isTauriAvailable(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export async function analyzePgnText(
  pgn: string,
  enginePath: string,
  depth: number = 14
): Promise<GameAnalysis> {
  if (!isTauriAvailable()) {
    if (import.meta.env.VITE_USE_MOCK_ANALYSIS === "true") {
      console.log("[TempoLens] Tauri not available, using mock analysis");
      await new Promise((resolve) => setTimeout(resolve, 500));
      return MOCK_OPERA_HOUSE_ANALYSIS;
    }
    throw new Error("Tauri is not available. Run the desktop app to analyze PGNs.");
  }

  const { invoke } = await import("@tauri-apps/api/core");
  const raw = await invoke<string>("analyze_pgn_text", { pgn, enginePath, depth });
  return normalizeAnalysisOutput(raw);
}

export async function analyzePgnFile(
  path: string,
  enginePath: string,
  depth: number = 14
): Promise<GameAnalysis> {
  if (!isTauriAvailable()) {
    if (import.meta.env.VITE_USE_MOCK_ANALYSIS === "true") {
      console.log("[TempoLens] Tauri not available, using mock analysis");
      await new Promise((resolve) => setTimeout(resolve, 500));
      return MOCK_OPERA_HOUSE_ANALYSIS;
    }
    throw new Error("Tauri is not available. Run the desktop app to analyze PGNs.");
  }

  const { invoke } = await import("@tauri-apps/api/core");
  const raw = await invoke<string>("analyze_pgn_file", { path, enginePath, depth });
  return normalizeAnalysisOutput(raw);
}

function normalizeAnalysisOutput(raw: string): GameAnalysis {
  const parsed = JSON.parse(raw) as GameAnalysis | GameAnalysis[];
  if (Array.isArray(parsed)) {
    if (parsed.length === 0) {
      throw new Error("Analysis returned no games.");
    }
    return parsed[0];
  }
  return parsed;
}
