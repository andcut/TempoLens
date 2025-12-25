import React from "react";

export type PieceType = "k" | "q" | "r" | "b" | "n" | "p";
export type PieceColor = "w" | "b";
export type SquarePiece = { type: PieceType; color: PieceColor } | null;

export function parseFen(fen: string): SquarePiece[][] {
    const [position] = fen.split(" ");
    const rows = position.split("/");
    const board: SquarePiece[][] = [];

    for (const row of rows) {
        const boardRow: SquarePiece[] = [];
        for (const char of row) {
            if (isNaN(parseInt(char))) {
                const type = char.toLowerCase() as PieceType;
                const color = char === char.toUpperCase() ? "w" : "b";
                boardRow.push({ type, color });
            } else {
                const emptyCount = parseInt(char);
                for (let i = 0; i < emptyCount; i++) {
                    boardRow.push(null);
                }
            }
        }
        board.push(boardRow);
    }
    return board;
}

export const PIECE_ORDER: PieceType[] = ["k", "q", "r", "b", "n", "p"];

export function getPieceIndex(type: PieceType): number {
    return PIECE_ORDER.indexOf(type);
}
