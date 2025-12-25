import React from "react";
import type { PlyAnalysis } from "../types";

interface Props {
  plies: PlyAnalysis[];
  selectedIndex: number;
  onSelect: (index: number) => void;
}

export function MoveList({ plies, selectedIndex, onSelect }: Props) {
  return (
    <div className="panel stack">
      <div className="panel-title">Moves</div>
      <div className="move-list">
        {plies.map((ply, idx) => (
          <button
            key={ply.ply.ply_index}
            className={`move ${idx === selectedIndex ? "active" : ""}`}
            onClick={() => onSelect(idx)}
          >
            <span className="move-index">{ply.ply.ply_index}</span>
            <span className="move-san">{ply.ply.san}</span>
            <span className="move-label">{ply.label.title}</span>
          </button>
        ))}
      </div>
    </div>
  );
}
