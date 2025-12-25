import React from "react";
import type { PlyAnalysis } from "../types";

interface Props {
  plies: PlyAnalysis[];
}

export function ClockGraph({ plies }: Props) {
  return (
    <div className="panel stack">
      <div className="panel-title">Clock Timeline</div>
      <div className="graph-container">
        <div className="graph-with-axis">
          <div className="axis-y">
            <span>60s</span>
            <span>30s</span>
            <span>0s</span>
          </div>
          <div className="sparkline">
            {plies.slice(0, 60).map((ply, idx) => {
              const t = ply.ply.clock_after_secs ?? 0;
              // Scale: 60px height = 60s
              const h = Math.max(6, Math.min(60, t));
              return (
                <span
                  key={idx}
                  style={{ height: `${h}px` }}
                  title={`${ply.ply.san}: ${t.toFixed(1)}s remaining`}
                />
              );
            })}
          </div>
        </div>
      </div>
      <div className="muted">Seconds remaining after each move.</div>
    </div>
  );
}
