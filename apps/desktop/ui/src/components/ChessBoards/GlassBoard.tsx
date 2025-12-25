import React from "react";
import { parseFen, SquarePiece } from "../../utils/chess";

interface Props {
    fen: string;
}

export function GlassBoard({ fen }: Props) {
    const board = parseFen(fen);

    return (
        <div className="glass-board">
            {board.map((row, i) => (
                <div key={i} className="board-row">
                    {row.map((piece, j) => (
                        <div
                            key={`${i}-${j}`}
                            className={`square ${(i + j) % 2 === 0 ? "light" : "dark"}`}
                        >
                            <div className="square-content">
                                {piece && <Piece piece={piece} />}
                            </div>
                        </div>
                    ))}
                </div>
            ))}
        </div>
    );
}

function Piece({ piece }: { piece: SquarePiece }) {
    if (!piece) return null;
    const { type, color } = piece;

    // Use a sleek SVG set or font-based pieces. 
    // For 'Glass', we want symbols that glow.
    const symbolMap: Record<string, string> = {
        wk: "K", wq: "Q", wr: "R", wb: "B", wn: "N", wp: "P",
        bk: "k", bq: "q", br: "r", bb: "b", bn: "n", bp: "p"
    };

    return (
        <div className={`glass-piece ${color}`}>
            {symbolMap[`${color}${type}`]}
        </div>
    );
}
