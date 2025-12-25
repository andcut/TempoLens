import React, { useEffect, useMemo, useState } from "react";
import { analyzePgnText } from "../api";
import { BoardViewV2 } from "../components/BoardViewV2";
import { ClockGraph } from "../components/ClockGraph";
import { GameLoader } from "../components/GameLoader";
import { MoveDetailPanel } from "../components/MoveDetailPanel";
import { MoveList } from "../components/MoveList";
import { PracticalScoreGraph } from "../components/PracticalScoreGraph";
import { OPERA_HOUSE_PGN } from "../mockData";
import type { GameAnalysis } from "../types";



export function AnalyzePage() {
  const [pgn, setPgn] = useState(OPERA_HOUSE_PGN);
  const [enginePath, setEnginePath] = useState("");
  const [analysis, setAnalysis] = useState<GameAnalysis | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [selectedIndex, setSelectedIndex] = useState(0);

  const selected = analysis?.plies[selectedIndex];
  const header = useMemo(() => {
    if (!analysis) return "No game loaded";
    const white = analysis.meta?.white ?? "White";
    const black = analysis.meta?.black ?? "Black";
    const result = analysis.meta?.result ?? "*";
    return `${white} vs ${black} · ${result}`;
  }, [analysis]);

  const onAnalyze = async () => {
    setBusy(true);
    setError(null);
    try {
      const res = await analyzePgnText(pgn, enginePath);
      setAnalysis(res);
      setSelectedIndex(0);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

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
                Moves analyzed: {analysis.plies.length} · <span className="depth-badge">Depth 14</span>
              </>
            ) : (
              "Paste a PGN to begin"
            )}
          </div>
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
          {error && <div className="panel error">{error}</div>}
        </div>

        <div className="column">
          <BoardViewV2 fen={selected?.ply.fen_before || ""} />
          <ClockGraph plies={analysis?.plies ?? []} />
        </div>

        <div className="column">
          <PracticalScoreGraph plies={analysis?.plies ?? []} />
          <MoveDetailPanel ply={selected} />
        </div>

        <div className="column wide">
          <MoveList
            plies={analysis?.plies ?? []}
            selectedIndex={selectedIndex}
            onSelect={setSelectedIndex}
          />
        </div>
      </section>
    </div>
  );
}
