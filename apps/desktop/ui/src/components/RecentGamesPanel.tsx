import React from "react";
import type { RecentPgnEntry } from "../types";

interface Props {
  entries: RecentPgnEntry[];
  onLoad: (entry: RecentPgnEntry) => void;
  onClear: () => void;
}

export function RecentGamesPanel({ entries, onLoad, onClear }: Props) {
  return (
    <div className="panel stack recent-panel">
      <div className="panel-title">Recent PGNs</div>
      {entries.length === 0 ? (
        <div className="muted">No recent games yet.</div>
      ) : (
        <div className="recent-list">
          {entries.map((entry) => (
            <button
              key={entry.id}
              className="recent-item"
              onClick={() => onLoad(entry)}
              title={new Date(entry.savedAt).toLocaleString()}
            >
              <span>{entry.label}</span>
              <span className="recent-date">
                {new Date(entry.savedAt).toLocaleDateString()}
              </span>
            </button>
          ))}
        </div>
      )}
      {entries.length > 0 && (
        <button className="ghost" onClick={onClear}>
          Clear list
        </button>
      )}
    </div>
  );
}
