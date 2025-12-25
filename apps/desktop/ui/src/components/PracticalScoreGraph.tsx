import React from "react";
import type { PlyAnalysis } from "../types";

interface Props {
  plies: PlyAnalysis[];
}

export function PracticalScoreGraph({ plies }: Props) {
  return (
    <div className="panel stack">
      <div className="panel-title">Practical Score</div>
      <div className="sparkline score">
        {plies.slice(0, 40).map((ply, idx) => {
          const p = ply.metrics.p_practical_after ?? 0.5;
          const h = Math.max(6, Math.min(60, p * 60));
          return <span key={idx} style={{ height: `${h}px` }} />;
        })}
      </div>
      <div className="muted">Shows practical win probability over time.</div>
    </div>
  );
}
