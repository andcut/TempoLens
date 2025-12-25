import React from "react";
import type { PlyAnalysis } from "../types";

interface Props {
  plies: PlyAnalysis[];
  onSelect?: (index: number) => void;
  selectedIndex?: number;
}

export function PracticalScoreGraph({ plies, onSelect, selectedIndex }: Props) {
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
          <div className="sparkline score clickable">
            {plies.slice(0, 80).map((ply, idx) => {
              const p = ply.metrics.p_practical_after ?? 0.5;
              const h = Math.max(4, p * 60);
              const isSelected = idx === selectedIndex;
              return (
                <span
                  key={idx}
                  className={isSelected ? "selected" : ""}
                  style={{ height: `${h}px` }}
                  title={`${ply.ply.san}: ${(p * 100).toFixed(1)}% win prob`}
                  onClick={() => onSelect?.(idx)}
                />
              );
            })}
          </div>
        </div>
      </div>
      <div className="muted">Practical win probability (0.0 to 1.0). Click to select.</div>
    </div>
  );
}
