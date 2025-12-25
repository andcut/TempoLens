import React, { useState } from "react";
import { parseFen, SquarePiece } from "../utils/chess";
import "../styles/boards_v2.css";

interface Props {
    fen: string;
}

type Theme = "wood" | "midnight" | "marble";

const PIECE_CHARS: Record<string, string> = {
    wK: "♔", wQ: "♕", wR: "♖", wB: "♗", wN: "♘", wP: "♙",
    bK: "♚", bQ: "♛", bR: "♜", bB: "♝", bN: "♞", bP: "♟",
};

export function BoardViewV2({ fen }: Props) {
    const [theme, setTheme] = useState<Theme>("wood");
    const board = fen ? parseFen(fen) : parseFen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    return (
        <div className="panel board-panel-v2">
            <div className="panel-header">
                <div className="panel-title">Board</div>
                <div className="theme-switcher">
                    <button className={theme === "wood" ? "active" : ""} onClick={() => setTheme("wood")}>Wood</button>
                    <button className={theme === "midnight" ? "active" : ""} onClick={() => setTheme("midnight")}>Midnight</button>
                    <button className={theme === "marble" ? "active" : ""} onClick={() => setTheme("marble")}>Marble</button>
                </div>
            </div>

            <div className={`board-wrapper theme-${theme}`}>
                <div className="chess-board">
                    {board.map((row, r) => (
                        <div key={r} className="board-row">
                            {row.map((piece, c) => {
                                const isLight = (r + c) % 2 === 0;
                                return (
                                    <div key={`${r}-${c}`} className={`square ${isLight ? "light" : "dark"}`}>
                                        {piece && (
                                            <span className={`piece ${piece.color === "w" ? "white-piece" : "black-piece"}`}>
                                                {PIECE_CHARS[`${piece.color}${piece.type.toUpperCase()}`]}
                                            </span>
                                        )}
                                    </div>
                                );
                            })}
                        </div>
                    ))}
                </div>
            </div>

            <div className="board-fen">{fen || "Starting position"}</div>
        </div>
    );
}
