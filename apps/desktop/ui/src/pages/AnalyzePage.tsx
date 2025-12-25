import React, { useEffect, useMemo, useState } from "react";
import { analyzePgnText } from "../api";
import { AboutModal } from "../components/AboutModal";
import { BoardViewV2 } from "../components/BoardViewV2";
import { ClockGraph } from "../components/ClockGraph";
import { GameLoader } from "../components/GameLoader";
import { MoveDetailPanel } from "../components/MoveDetailPanel";
import { MoveList } from "../components/MoveList";
import { PracticalScoreGraph } from "../components/PracticalScoreGraph";
import { RecentGamesPanel } from "../components/RecentGamesPanel";
import { SettingsPanel } from "../components/SettingsPanel";
import { SummaryPanel } from "../components/SummaryPanel";
import { OPERA_HOUSE_PGN } from "../mockData";
import type { AnalysisOptions, GameAnalysis, RecentPgnEntry } from "../types";
import { useLocalStorage } from "../utils/useLocalStorage";
import { formatPgnLabel, parsePgnMeta } from "../utils/pgnMeta";

export function AnalyzePage() {
  const [pgn, setPgn] = useState(OPERA_HOUSE_PGN);
  const [enginePath, setEnginePath] = useLocalStorage("enginePath", "/opt/homebrew/bin/stockfish");
  const [options, setOptions] = useLocalStorage<AnalysisOptions>("analysisOptions", {
    depth: 14,
    multipv: 4,
    movetime_ms: null,
    threads: null,
    hash_mb: null,
    time_control: null,
    alpha: 2.0,
    beta: 10.0,
    time_pressure_pivot: 30.0,
    time_pressure_scale: 8.0,
    time_pressure_boost: 3.0,
    k_sigmoid: 1.2
  });
  const [recentGames, setRecentGames] = useLocalStorage<RecentPgnEntry[]>("recentPgns", []);
  const [analysis, setAnalysis] = useState<GameAnalysis | null>(null);
  const [busy, setBusy] = useState(false);
  const [progress, setProgress] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [aboutOpen, setAboutOpen] = useState(false);
  const [appVersion, setAppVersion] = useState<string | null>(null);

  const selected = analysis?.plies[selectedIndex];

  const header = useMemo(() => {
    if (!analysis) return "No game loaded";
    const white = analysis.meta?.white ?? "White";
    const black = analysis.meta?.black ?? "Black";
    const result = analysis.meta?.result ?? "*";
    return `${white} vs ${black} · ${result}`;
  }, [analysis]);

  const timeControlLabel = useMemo(() => {
    if (!analysis?.meta?.time_control) return null;
    const tc = analysis.meta.time_control;
    return `${tc.base_secs / 60}+${tc.increment_secs}`;
  }, [analysis]);

  const onAnalyze = async () => {
    if (!enginePath.trim()) {
      setError("Engine path is required.");
      return;
    }
    setBusy(true);
    setError(null);
    setProgress("Starting analysis...");
    try {
      const res = await analyzePgnText(pgn, enginePath, options);
      setAnalysis(res);
      setSelectedIndex(0);
      setProgress(null);
      const meta = parsePgnMeta(pgn);
      const entry: RecentPgnEntry = {
        id: `${Date.now()}`,
        label: formatPgnLabel(meta),
        savedAt: Date.now(),
        pgn
      };
      setRecentGames((prev) => [entry, ...prev.filter((item) => item.pgn !== pgn)].slice(0, 6));
    } catch (err) {
      const errorStr = String(err);
      // Parse common errors into friendly messages
      if (errorStr.includes("No such file or directory")) {
        setError("Engine not found. Please check the engine path.");
      } else if (errorStr.includes("Invalid PGN")) {
        setError("Invalid PGN format. Please check your input.");
      } else {
        setError(errorStr.replace(/^Error:\s*/, ""));
      }
      setProgress(null);
    } finally {
      setBusy(false);
    }
  };

  const exportAnalysis = () => {
    if (!analysis) return;
    const blob = new Blob([JSON.stringify(analysis, null, 2)], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `tempolens-analysis-${Date.now()}.json`;
    a.click();
    URL.revokeObjectURL(url);
  };

  // Keyboard navigation
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!analysis) return;
      if (e.key === "ArrowRight") {
        setSelectedIndex((prev) => Math.min(prev + 1, analysis.plies.length - 1));
      } else if (e.key === "ArrowLeft") {
        setSelectedIndex((prev) => Math.max(prev - 1, 0));
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [analysis]);

  useEffect(() => {
    const handleSettingsEvent = async () => {
      setSettingsOpen(true);
      const el = document.querySelector(".settings-panel");
      if (el) {
        el.scrollIntoView({ behavior: "smooth", block: "center" });
      }
    };

    const bindEvents = async () => {
      if (typeof window === "undefined" || !("__TAURI_INTERNALS__" in window)) return;
      const { listen } = await import("@tauri-apps/api/event");
      const unlisten = await listen("open-settings", handleSettingsEvent);
      const unlistenAbout = await listen("open-about", () => setAboutOpen(true));
      return () => {
        unlisten();
        unlistenAbout();
      };
    };

    const cleanup = bindEvents();
    return () => {
      if (cleanup instanceof Promise) {
        cleanup.then((fn) => fn && fn());
      }
    };
  }, []);

  useEffect(() => {
    const loadVersion = async () => {
      if (typeof window === "undefined" || !("__TAURI_INTERNALS__" in window)) return;
      const { getVersion } = await import("@tauri-apps/api/app");
      const version = await getVersion();
      setAppVersion(version);
    };
    loadVersion();
  }, []);

  return (
    <div className="app">
      <header className="hero">
        <div>
          <div className="kicker">TempoLens</div>
          <h1>Time is the hidden evaluation.</h1>
          <p>
            Measure clock pressure, convert it into pawn-equity, and diagnose where you burned
            the game faster than the board.
          </p>
        </div>
        <div className="hero-card">
          <div className="hero-title">{header}</div>
          <div className="hero-meta">
            {analysis ? (
              <>
                {analysis.plies.length} moves ·
                <span className="depth-badge">Depth {analysis.plies[0]?.engine_before?.depth ?? options.depth}</span>
                {timeControlLabel && <span className="tc-badge">{timeControlLabel}</span>}
              </>
            ) : (
              "Paste a PGN to begin"
            )}
          </div>
          {analysis && (
            <button className="cta-small export-btn" onClick={exportAnalysis}>
              Export JSON
            </button>
          )}
          <button className="ghost about-btn" onClick={() => setAboutOpen(true)}>
            About TempoLens
          </button>
        </div>
      </header>

      <section className="grid">
        <div className="column">
          <GameLoader
            pgn={pgn}
            enginePath={enginePath}
            onChangePgn={setPgn}
            onChangeEnginePath={setEnginePath}
            onAnalyze={onAnalyze}
            isBusy={busy}
          />
          <button
            className="ghost settings-toggle"
            onClick={() => setSettingsOpen((prev) => !prev)}
          >
            {settingsOpen ? "Hide settings" : "Show settings"}
          </button>
          {settingsOpen && (
            <SettingsPanel
              options={options}
              onChange={setOptions}
              onReset={() =>
                setOptions({
                  depth: 14,
                  multipv: 4,
                  movetime_ms: null,
                  threads: null,
                  hash_mb: null,
                  time_control: null,
                  alpha: 2.0,
                  beta: 10.0,
                  time_pressure_pivot: 30.0,
                  time_pressure_scale: 8.0,
                  time_pressure_boost: 3.0,
                  k_sigmoid: 1.2
                })
              }
            />
          )}
          <RecentGamesPanel
            entries={recentGames}
            onLoad={(entry) => setPgn(entry.pgn)}
            onClear={() => setRecentGames([])}
          />
          {progress && <div className="panel progress">{progress}</div>}
          {error && <div className="panel error">{error}</div>}
        </div>

        <div className="column">
          <BoardViewV2 fen={selected?.ply.fen_before || ""} />
          <ClockGraph
            plies={analysis?.plies ?? []}
            timeControl={analysis?.meta?.time_control}
            onSelect={setSelectedIndex}
            selectedIndex={selectedIndex}
          />
        </div>

        <div className="column">
          <PracticalScoreGraph
            plies={analysis?.plies ?? []}
            onSelect={setSelectedIndex}
            selectedIndex={selectedIndex}
          />
          <MoveDetailPanel ply={selected} />
        </div>

        <div className="column wide">
          <MoveList
            plies={analysis?.plies ?? []}
            selectedIndex={selectedIndex}
            onSelect={setSelectedIndex}
          />
          {analysis?.summary && <SummaryPanel summary={analysis.summary} />}
        </div>
      </section>
      <AboutModal open={aboutOpen} version={appVersion} onClose={() => setAboutOpen(false)} />
    </div>
  );
}
