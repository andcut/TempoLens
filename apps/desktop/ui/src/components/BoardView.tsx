import React, { useState } from "react";
import { MinimalBoard } from "./ChessBoards/MinimalBoard";
import { LuxuryBoard } from "./ChessBoards/LuxuryBoard";
import { GlassBoard } from "./ChessBoards/GlassBoard";

interface Props {
  fen?: string;
}

type Variant = "minimal" | "luxury" | "glass";

export function BoardView({ fen = "rnbqkbnr/pppppppp/8/8/8/8/pppppppp/RNBQKBNR w KQkq - 0 1" }: Props) {
  const [variant, setVariant] = useState<Variant>("luxury");

  return (
    <div className="panel board-panel">
      <div className="panel-header">
        <div className="panel-title">Board Design</div>
        <div className="board-switcher">
          <button
            className={variant === "minimal" ? "active" : ""}
            onClick={() => setVariant("minimal")}
          >
            Minimal
          </button>
          <button
            className={variant === "luxury" ? "active" : ""}
            onClick={() => setVariant("luxury")}
          >
            Luxury
          </button>
          <button
            className={variant === "glass" ? "active" : ""}
            onClick={() => setVariant("glass")}
          >
            Glass
          </button>
        </div>
      </div>

      <div className="board-container">
        {variant === "minimal" && <MinimalBoard fen={fen} />}
        {variant === "luxury" && <LuxuryBoard fen={fen} />}
        {variant === "glass" && <GlassBoard fen={fen} />}
      </div>

      <div className="board-info">
        <div className="board-fen">{fen}</div>
      </div>
    </div>
  );
}

