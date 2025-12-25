import React from "react";
import { parseFen, SquarePiece } from "../../utils/chess";

interface Props {
    fen: string;
}

export function MinimalBoard({ fen }: Props) {
    const board = parseFen(fen);

    return (
        <div className="minimal-board">
            {board.map((row, i) => (
                <div key={i} className="board-row">
                    {row.map((piece, j) => (
                        <div
                            key={`${i}-${j}`}
                            className={`square ${(i + j) % 2 === 0 ? "light" : "dark"}`}
                        >
                            {piece && <Piece piece={piece} />}
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

    // Minimal representation using Unicode for maximum reliability and simplicity
    const unicodeMap: Record<string, string> = {
        wk: "♔", wq: "♕", wr: "♖", wb: "♗", wn: "♘", wp: "♙",
        bk: "♚", bq: "♛", br: "♜", bb: "♝", bn: "♞", bp: "♟"
    };

    return <span className={`piece ${color}`}>{unicodeMap[`${color}${type}`]}</span>;
}
