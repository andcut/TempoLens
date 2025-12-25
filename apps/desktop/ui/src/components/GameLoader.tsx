import React from "react";

interface Props {
  pgn: string;
  enginePath: string;
  onChangePgn: (value: string) => void;
  onChangeEnginePath: (value: string) => void;
  onAnalyze: () => void;
  isBusy: boolean;
}

export function GameLoader({
  pgn,
  enginePath,
  onChangePgn,
  onChangeEnginePath,
  onAnalyze,
  isBusy
}: Props) {
  return (
    <div className="panel stack">
      <div className="panel-title">Input</div>
      <label className="field">
        <span>Engine path</span>
        <input
          value={enginePath}
          onChange={(e) => onChangeEnginePath(e.target.value)}
          placeholder="/path/to/stockfish"
        />
      </label>
      <label className="field">
        <span>PGN</span>
        <textarea
          value={pgn}
          onChange={(e) => onChangePgn(e.target.value)}
          placeholder="Paste PGN with [%clk ...] comments"
          rows={10}
        />
      </label>
      <button className="cta" onClick={onAnalyze} disabled={isBusy}>
        {isBusy ? "Analyzing..." : "Analyze"}
      </button>
    </div>
  );
}
