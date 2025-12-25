import React from "react";
import type { PlyAnalysis } from "../types";

interface Props {
  ply?: PlyAnalysis;
}

export function MoveDetailPanel({ ply }: Props) {
  if (!ply) {
    return (
      <div className="panel stack">
        <div className="panel-title">Detail</div>
        <div className="muted">Select a move to see details.</div>
      </div>
    );
  }

  return (
    <div className="panel stack">
      <div className="panel-title">Detail</div>
      <div className="detail-header">
        <div className="detail-move">{ply.ply.san}</div>
        <div className="badge">{ply.label.title}</div>
      </div>
      <div className="detail-body">
        <p>{ply.label.explanation}</p>
        <div className="detail-grid">
          <div>
            <span>Eval Δ</span>
            <strong>{ply.metrics.dp_eval_mover.toFixed(3)}</strong>
          </div>
          <div>
            <span>Practical Δ</span>
            <strong>{ply.metrics.dp_practical_mover.toFixed(3)}</strong>
          </div>
          <div>
            <span>Think time</span>
            <strong>{(ply.ply.think_time_secs ?? 0).toFixed(1)}s</strong>
          </div>
        </div>
        <div className="tips">
          {ply.label.tips.map((tip, idx) => (
            <div key={idx}>• {tip}</div>
          ))}
        </div>
      </div>
    </div>
  );
}
