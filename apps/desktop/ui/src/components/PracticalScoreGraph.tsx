import React from "react";
import type { PlyAnalysis } from "../types";

interface Props {
  plies: PlyAnalysis[];
}

export function PracticalScoreGraph({ plies }: Props) {
  return (
    <div className="panel stack">
      <div className="panel-title">Practical Score</div>
      <div className="graph-container">
        <div className="graph-with-axis">
          <div className="axis-y">
            <span>1.0</span>
            <span>0.5</span>
            <span>0.0</span>
          </div>
          <div className="sparkline score">
            {plies.slice(0, 60).map((ply, idx) => {
              const p = ply.metrics.p_practical_after ?? 0.5;
              const h = Math.max(6, Math.min(60, p * 60));
              return (
                <span
                  key={idx}
                  style={{ height: `${h}px` }}
                  title={`${ply.ply.san}: ${(p * 100).toFixed(1)}% win prob`}
                />
              );
            })}
          </div>
        </div>
      </div>
      <div className="muted">Practical win probability (0.0 to 1.0) over time.</div>
    </div>
  );
}
