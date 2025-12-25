import React from "react";
import type { GameSummary } from "../types";

interface Props {
    summary: GameSummary;
}

export function SummaryPanel({ summary }: Props) {
    const formatPercent = (n?: number | null) =>
        n != null ? `${(n * 100).toFixed(1)}%` : "—";

    const formatTime = (n?: number | null) =>
        n != null ? `${n.toFixed(1)}s` : "—";

    return (
        <div className="panel stack summary-panel">
            <div className="panel-title">Game Summary</div>

            <div className="summary-grid">
                <div className="summary-section">
                    <h4>Time Management</h4>
                    <div className="stat-row">
                        <span>Avg think time</span>
                        <strong>{formatTime(summary.avg_think_time_secs)}</strong>
                    </div>
                    <div className="stat-row">
                        <span>Time trouble moves</span>
                        <strong>{summary.time_trouble_moves} ({formatPercent(summary.time_trouble_rate)})</strong>
                    </div>
                    <div className="stat-row">
                        <span>Panic moves</span>
                        <strong>{summary.panic_moves} ({formatPercent(summary.panic_rate)})</strong>
                    </div>
                    <div className="stat-row">
                        <span>Blunders in time trouble</span>
                        <strong>{summary.blunders_in_time_trouble}</strong>
                    </div>
                </div>

                <div className="summary-section">
                    <h4>Phase Allocation</h4>
                    <div className="phase-bars">
                        <div className="phase-bar">
                            <span>Opening</span>
                            <div className="bar-track">
                                <div
                                    className="bar-fill opening"
                                    style={{ width: `${(summary.phase_time_share?.opening ?? 0) * 100}%` }}
                                />
                            </div>
                            <span className="phase-pct">{formatPercent(summary.phase_time_share?.opening)}</span>
                        </div>
                        <div className="phase-bar">
                            <span>Middle</span>
                            <div className="bar-track">
                                <div
                                    className="bar-fill middlegame"
                                    style={{ width: `${(summary.phase_time_share?.middlegame ?? 0) * 100}%` }}
                                />
                            </div>
                            <span className="phase-pct">{formatPercent(summary.phase_time_share?.middlegame)}</span>
                        </div>
                        <div className="phase-bar">
                            <span>Endgame</span>
                            <div className="bar-track">
                                <div
                                    className="bar-fill endgame"
                                    style={{ width: `${(summary.phase_time_share?.endgame ?? 0) * 100}%` }}
                                />
                            </div>
                            <span className="phase-pct">{formatPercent(summary.phase_time_share?.endgame)}</span>
                        </div>
                    </div>
                </div>

                <div className="summary-section">
                    <h4>Quality Metrics</h4>
                    <div className="stat-row">
                        <span>Avg ΔP (practical)</span>
                        <strong>{summary.avg_dp_practical_mover != null
                            ? `${(summary.avg_dp_practical_mover * 100).toFixed(2)}%`
                            : "—"}</strong>
                    </div>
                    <div className="stat-row">
                        <span>Avg punish</span>
                        <strong>{summary.avg_punish_cp_mover != null
                            ? `${summary.avg_punish_cp_mover.toFixed(0)} cp`
                            : "—"}</strong>
                    </div>
                    <div className="stat-row">
                        <span>Avg complexity</span>
                        <strong>{summary.avg_complexity_cp_mover != null
                            ? `${summary.avg_complexity_cp_mover.toFixed(0)} cp`
                            : "—"}</strong>
                    </div>
                </div>
            </div>
        </div>
    );
}
