import React from "react";
import type { PlyAnalysis } from "../types";

interface Props {
  plies: PlyAnalysis[];
}

export function ClockGraph({ plies }: Props) {
  return (
    <div className="panel stack">
      <div className="panel-title">Clock Timeline</div>
      <div className="sparkline">
        {plies.slice(0, 40).map((ply, idx) => {
          const t = ply.ply.clock_after_secs ?? 0;
          const h = Math.max(6, Math.min(60, t / 4));
          return <span key={idx} style={{ height: `${h}px` }} />;
        })}
      </div>
      <div className="muted">Bars approximate remaining time after each move.</div>
    </div>
  );
}
