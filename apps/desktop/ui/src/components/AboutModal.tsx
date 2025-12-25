import React from "react";

interface Props {
  open: boolean;
  version?: string | null;
  onClose: () => void;
}

export function AboutModal({ open, version, onClose }: Props) {
  if (!open) return null;

  return (
    <div className="modal-backdrop" onClick={onClose}>
      <div className="modal" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h3>About TempoLens</h3>
          <button className="ghost" onClick={onClose}>Close</button>
        </div>
        <div className="modal-body">
          <p>TempoLens is a time-management analysis tool for blitz and bullet chess.</p>
          <div className="modal-meta">
            <div>
              <span>Version</span>
              <strong>{version ?? "Unknown"}</strong>
            </div>
            <div>
              <span>Engine</span>
              <strong>Stockfish (GPLv3)</strong>
            </div>
          </div>
          <p className="muted">
            If you distribute Stockfish binaries, include GPL text and source access. See
            docs/licenses/stockfish.md.
          </p>
        </div>
      </div>
    </div>
  );
}
