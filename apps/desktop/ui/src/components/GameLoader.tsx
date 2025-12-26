import React, { useState } from "react";

// Classic blitz/bullet games for quick analysis - all Lichess URLs for direct fetch
const SAMPLE_GAMES = [
  {
    label: "Select a classic game...",
    url: "",
    description: ""
  },
  {
    label: "ChessTrain86 vs GM Smeets (Bullet)",
    url: "https://lichess.org/ztzR1jWj",
    description: "Reversed Grünfeld, long rook endgame won on time"
  },
  {
    label: "SelfmateMan 'Nearly Immortal' (Bullet)",
    url: "https://lichess.org/zbkqbYib",
    description: "IM beaten, missed mate in 7, loses on time winning"
  },
  {
    label: "Toscani's 62-Move Scramble (Bullet)",
    url: "https://lichess.org/TmrFj2ckxWHD",
    description: "Pure premove stamina demo"
  },
  {
    label: "95-Move Flagfest (Bullet)",
    url: "https://lichess.org/qdY7JUHr",
    description: "Textbook bullet cruelty - milking a won position"
  },
  {
    label: "Double Berserk Marathon (Bullet)",
    url: "https://lichess.org/AuBLE9zB",
    description: "Both berserk, ultra-sharp tactical fight"
  },
  {
    label: "Wild Hyperbullet Race",
    url: "https://lichess.org/gjY9gVks",
    description: "Chaotic tactics and time-scramble finish"
  },
  {
    label: "DasKaninchen Flag & Mate (Hyperbullet)",
    url: "https://lichess.org/a7HflAkxALcY",
    description: "77-move speed-tactics demo"
  },
  {
    label: "Hyperbullet Epic (8Wof60H2)",
    url: "https://lichess.org/8Wof60H2qsWP",
    description: "Long tactical slugfest"
  },
  {
    label: "'Pure Anger' 95% Accuracy (Blitz)",
    url: "https://lichess.org/CHLFVtVS",
    description: "5-min game in 1:20, rage-driven miniature"
  },
  {
    label: "Mugiwara's Immortal (Bullet)",
    url: "https://lichess.org/BzLomQyv",
    description: "Flashy attacking bullet"
  }
];

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

  const handleSampleSelect = (url: string) => {
    if (url) {
      setUrlInput(url);
      setUrlError(null);
    }
  };

  const handleFetchUrl = async () => {
    if (!urlInput.trim()) return;

    setFetchingUrl(true);
    setUrlError(null);

    try {
      let gameId = "";

      // Parse Lichess game URL (e.g. lichess.org/ztzR1jWj/black)
      const lichessGameMatch = urlInput.match(/lichess\.org\/([a-zA-Z0-9]{8,12})/);
      if (lichessGameMatch) {
        gameId = lichessGameMatch[1];
        const response = await fetch(`https://lichess.org/game/export/${gameId}?clocks=true`);
        if (!response.ok) throw new Error("Failed to fetch from Lichess");
        const pgnText = await response.text();
        onChangePgn(pgnText);
        setUrlInput("");
        setFetchingUrl(false);
        return;
      }

      // Parse Lichess study URL (e.g. lichess.org/study/GGbayufJ/sYfxYD1Q)
      const lichessStudyMatch = urlInput.match(/lichess\.org\/study\/([a-zA-Z0-9]+)\/([a-zA-Z0-9]+)/);
      if (lichessStudyMatch) {
        const studyId = lichessStudyMatch[1];
        const chapterId = lichessStudyMatch[2];
        const response = await fetch(`https://lichess.org/api/study/${studyId}/${chapterId}.pgn`);
        if (!response.ok) throw new Error("Failed to fetch study from Lichess");
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

      // Check for chessgames.com or article URLs
      if (urlInput.includes("chessgames.com") || urlInput.includes("chessdom.com") || urlInput.includes("chess.com/article")) {
        setUrlError("This URL cannot be fetched directly. Please visit the page and paste the PGN.");
        setFetchingUrl(false);
        return;
      }

      setUrlError("Unrecognized URL format. Supports: lichess.org games & studies");
    } catch (err) {
      setUrlError(String(err));
    } finally {
      setFetchingUrl(false);
    }
  };

  return (
    <div className="panel stack">
      <div className="panel-title">Input</div>

      {/* Sample Games Dropdown */}
      <label className="field">
        <span>Classic Games</span>
        <select
          onChange={(e) => handleSampleSelect(e.target.value)}
          defaultValue=""
          disabled={fetchingUrl}
        >
          {SAMPLE_GAMES.map((game, i) => (
            <option key={i} value={game.url} title={game.description}>
              {game.label}
            </option>
          ))}
        </select>
      </label>

      {/* URL Import */}
      <label className="field">
        <span>Import from URL</span>
        <div className="input-with-button">
          <input
            value={urlInput}
            onChange={(e) => setUrlInput(e.target.value)}
            placeholder="https://lichess.org/GAMEID or study URL"
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
