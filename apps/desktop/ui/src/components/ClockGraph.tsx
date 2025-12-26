import React from "react";
import type { PlyAnalysis, TimeControl } from "../types";

interface Props {
  plies: PlyAnalysis[];
  timeControl?: TimeControl | null;
  onSelect?: (index: number) => void;
  selectedIndex?: number;
}

export function ClockGraph({ plies, timeControl, onSelect, selectedIndex }: Props) {
  // Determine max time from time control or find max in data
  const maxTime = timeControl?.base_secs
    ? Math.min(timeControl.base_secs, 600) // Cap at 10 min for display
    : Math.max(60, ...plies.map(p => p.ply.clock_after_secs ?? 0));

  const formatAxisLabel = (secs: number) => {
    if (secs >= 60) return `${Math.round(secs / 60)}m`;
    return `${secs}s`;
  };

  return (
    <div className="panel stack">
      <div className="panel-title">Clock Timeline</div>
      <div className="graph-container">
        <div className="graph-with-axis">
          <div className="axis-y">
            <span>{formatAxisLabel(maxTime)}</span>
            <span>{formatAxisLabel(maxTime / 2)}</span>
            <span>0s</span>
          </div>
          <div className="sparkline clickable">
            {plies.map((ply, idx) => {
              const t = ply.ply.clock_after_secs ?? 0;
              const h = Math.max(4, (t / maxTime) * 60);
              const isSelected = idx === selectedIndex;
              return (
                <span
                  key={idx}
                  className={isSelected ? "selected" : ""}
                  style={{ height: `${h}px` }}
                  title={`${ply.ply.san}: ${t.toFixed(1)}s remaining`}
                  onClick={() => onSelect?.(idx)}
                />
              );
            })}
          </div>
        </div>
      </div>
      <div className="muted">Seconds remaining after each move. Click to select.</div>
    </div>
  );
}
