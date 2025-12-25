import React from "react";
import { parseFen, SquarePiece, getPieceIndex } from "../../utils/chess";

interface Props {
    fen: string;
}

const BOARD_IMG = "/assets/empty_luxury_board_texture_1766691791452.png";
const WHITE_PIECES = "/assets/white_marble_chess_pieces_1766691767438.png";
const BLACK_PIECES = "/assets/black_obsidian_chess_pieces_1766691779049.png";

export function LuxuryBoard({ fen }: Props) {
    const board = parseFen(fen);

    return (
        <div
            className="luxury-board"
            style={{ backgroundImage: `url(${BOARD_IMG})` }}
        >
            {board.map((row, i) => (
                <div key={i} className="board-row">
                    {row.map((piece, j) => (
                        <div key={`${i}-${j}`} className="square">
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
    const index = getPieceIndex(type);
    const spriteUrl = color === "w" ? WHITE_PIECES : BLACK_PIECES;

    return (
        <div
            className="piece-image"
            style={{
                backgroundImage: `url(${spriteUrl})`,
                backgroundSize: "600% 100%", // 6 pieces in a row
                backgroundPosition: `${(index / 5) * 100}% 0%` // index / (total-1)
            }}
        />
    );
}
