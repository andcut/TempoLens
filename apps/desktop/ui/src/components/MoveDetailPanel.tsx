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
            <span>Eval</span>
            <strong>{(ply.metrics.cp_eval_after / 100).toFixed(2)}</strong>
            <span className="sub-detail">Δ {ply.metrics.dp_eval_mover > 0 ? "+" : ""}{(ply.metrics.dp_eval_mover * 100).toFixed(1)}%</span>
          </div>
          <div>
            <span>Practical Prob</span>
            <strong>{(ply.metrics.p_practical_after * 100).toFixed(1)}%</strong>
            <span className="sub-detail">Δ {ply.metrics.dp_practical_mover > 0 ? "+" : ""}{(ply.metrics.dp_practical_mover * 100).toFixed(1)}%</span>
          </div>
          <div>
            <span>Think Time</span>
            <strong>{(ply.ply.think_time_secs ?? 0).toFixed(1)}s</strong>
            <span className="sub-detail">Rem: {(ply.ply.clock_after_secs ?? 0).toFixed(0)}s</span>
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
