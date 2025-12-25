import React, { useState } from "react";

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
  const [urlInput, setUrlInput] = useState("");
  const [urlError, setUrlError] = useState<string | null>(null);
  const [fetchingUrl, setFetchingUrl] = useState(false);

  const handleFetchUrl = async () => {
    if (!urlInput.trim()) return;

    setFetchingUrl(true);
    setUrlError(null);

    try {
      let gameId = "";

      // Parse Lichess URL
      const lichessMatch = urlInput.match(/lichess\.org\/([a-zA-Z0-9]{8,12})/);
      if (lichessMatch) {
        gameId = lichessMatch[1];
        const response = await fetch(`https://lichess.org/game/export/${gameId}?clocks=true`);
        if (!response.ok) throw new Error("Failed to fetch from Lichess");
        const pgnText = await response.text();
        onChangePgn(pgnText);
        setUrlInput("");
        setFetchingUrl(false);
        return;
      }

      // Parse Chess.com URL
      const chesscomMatch = urlInput.match(/chess\.com\/(?:game\/)?(?:live|daily)\/(\d+)/);
      if (chesscomMatch) {
        setUrlError("Chess.com import coming soon. Please paste the PGN directly.");
        setFetchingUrl(false);
        return;
      }

      setUrlError("Unrecognized URL format. Supports: lichess.org/GAMEID");
    } catch (err) {
      setUrlError(String(err));
    } finally {
      setFetchingUrl(false);
    }
  };

  return (
    <div className="panel stack">
      <div className="panel-title">Input</div>

      {/* URL Import */}
      <label className="field">
        <span>Import from URL</span>
        <div className="input-with-button">
          <input
            value={urlInput}
            onChange={(e) => setUrlInput(e.target.value)}
            placeholder="https://lichess.org/GAMEID"
            disabled={fetchingUrl}
          />
          <button
            className="cta-small"
            onClick={handleFetchUrl}
            disabled={fetchingUrl || !urlInput.trim()}
          >
            {fetchingUrl ? "..." : "Fetch"}
          </button>
        </div>
        {urlError && <span className="error-text">{urlError}</span>}
      </label>

      {/* Engine Settings */}
      <div className="field-row">
        <label className="field flex-grow">
          <span>Engine Path</span>
          <input
            value={enginePath}
            onChange={(e) => onChangeEnginePath(e.target.value)}
            placeholder="/opt/homebrew/bin/stockfish"
          />
        </label>
      </div>

      {/* PGN Input */}
      <label className="field">
        <span>PGN</span>
        <textarea
          value={pgn}
          onChange={(e) => onChangePgn(e.target.value)}
          placeholder="Paste PGN with [%clk ...] comments"
          rows={8}
        />
      </label>

      <button className="cta" onClick={onAnalyze} disabled={isBusy}>
        {isBusy ? "Analyzing..." : "Analyze"}
      </button>
    </div>
  );
}
