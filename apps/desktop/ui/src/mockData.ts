// Mock data for the Opera House Game (Morphy vs Duke of Brunswick and Count Isouard, 1858)
// Simulated as a 1+0 bullet game for demonstration

import type { GameAnalysis, PlyAnalysis, Label, MoveMetrics, PlyRecord } from "./types";

// The Opera House Game PGN with synthetic bullet clock data
export const OPERA_HOUSE_PGN = `[Event "Paris Opera House"]
[Site "Paris FRA"]
[Date "1858.11.02"]
[Round "?"]
[White "Morphy, Paul"]
[Black "Duke of Brunswick and Count Isouard"]
[Result "1-0"]
[TimeControl "60+0"]

1. e4 { [%clk 0:00:59] } e5 { [%clk 0:00:58] }
2. Nf3 { [%clk 0:00:57] } d6 { [%clk 0:00:55] }
3. d4 { [%clk 0:00:55] } Bg4 { [%clk 0:00:52] }
4. dxe5 { [%clk 0:00:53] } Bxf3 { [%clk 0:00:48] }
5. Qxf3 { [%clk 0:00:51] } dxe5 { [%clk 0:00:45] }
6. Bc4 { [%clk 0:00:49] } Nf6 { [%clk 0:00:42] }
7. Qb3 { [%clk 0:00:46] } Qe7 { [%clk 0:00:38] }
8. Nc3 { [%clk 0:00:44] } c6 { [%clk 0:00:34] }
9. Bg5 { [%clk 0:00:41] } b5 { [%clk 0:00:28] }
10. Nxb5 { [%clk 0:00:38] } cxb5 { [%clk 0:00:24] }
11. Bxb5+ { [%clk 0:00:35] } Nbd7 { [%clk 0:00:19] }
12. O-O-O { [%clk 0:00:32] } Rd8 { [%clk 0:00:14] }
13. Rxd7 { [%clk 0:00:28] } Rxd7 { [%clk 0:00:10] }
14. Rd1 { [%clk 0:00:25] } Qe6 { [%clk 0:00:06] }
15. Bxd7+ { [%clk 0:00:22] } Nxd7 { [%clk 0:00:03] }
16. Qb8+ { [%clk 0:00:19] } Nxb8 { [%clk 0:00:01] }
17. Rd8# { [%clk 0:00:16] } 1-0`;

// Helper to create a label
function makeLabel(kind: string, severity: number, title: string, explanation: string): Label {
    return { kind, severity, title, explanation, tips: [] };
}

// Helper to create metrics with sensible defaults
function makeMetrics(
    cpBefore: number,
    cpAfter: number,
    tauWhite: number = 0,
    dpEval: number = 0
): MoveMetrics {
    const pEvalBefore = 0.5 + 0.5 * (2 / (1 + Math.exp(-cpBefore / 400)) - 1);
    const pEvalAfter = 0.5 + 0.5 * (2 / (1 + Math.exp(-cpAfter / 400)) - 1);
    return {
        tau_white_cp: tauWhite,
        cp_eval_before: cpBefore,
        cp_eval_after: cpAfter,
        cp_practical_before: cpBefore + tauWhite,
        cp_practical_after: cpAfter + tauWhite,
        p_eval_before: pEvalBefore,
        p_eval_after: pEvalAfter,
        p_practical_before: pEvalBefore,
        p_practical_after: pEvalAfter,
        dp_eval_mover: dpEval,
        dp_practical_mover: dpEval,
    };
}

// Create mock analysis for Opera House Game
function createOperaHouseAnalysis(): GameAnalysis {
    const moves = [
        // Move, FEN before, FEN after, White clock, Black clock, eval before, eval after, label
        { san: "e4", uci: "e2e4", fenBefore: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", fenAfter: "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1", wClk: 59, bClk: 60, cpBefore: 20, cpAfter: 20, label: "Neutral" },
        { san: "e5", uci: "e7e5", fenBefore: "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1", fenAfter: "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2", wClk: 59, bClk: 58, cpBefore: 20, cpAfter: 20, label: "Neutral" },
        { san: "Nf3", uci: "g1f3", fenBefore: "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2", fenAfter: "rnbqkbnr/pppp1ppp/8/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2", wClk: 57, bClk: 58, cpBefore: 20, cpAfter: 25, label: "Neutral" },
        { san: "d6", uci: "d7d6", fenBefore: "rnbqkbnr/pppp1ppp/8/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2", fenAfter: "rnbqkbnr/ppp2ppp/3p4/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 3", wClk: 57, bClk: 55, cpBefore: 25, cpAfter: 45, label: "Neutral" },
        { san: "d4", uci: "d2d4", fenBefore: "rnbqkbnr/ppp2ppp/3p4/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 3", fenAfter: "rnbqkbnr/ppp2ppp/3p4/4p3/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 3", wClk: 55, bClk: 55, cpBefore: 45, cpAfter: 50, label: "Neutral" },
        { san: "Bg4", uci: "c8g4", fenBefore: "rnbqkbnr/ppp2ppp/3p4/4p3/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 3", fenAfter: "rn1qkbnr/ppp2ppp/3p4/4p3/3PP1b1/5N2/PPP2PPP/RNBQKB1R w KQkq - 1 4", wClk: 55, bClk: 52, cpBefore: 50, cpAfter: 80, label: "Neutral" },
        { san: "dxe5", uci: "d4e5", fenBefore: "rn1qkbnr/ppp2ppp/3p4/4p3/3PP1b1/5N2/PPP2PPP/RNBQKB1R w KQkq - 1 4", fenAfter: "rn1qkbnr/ppp2ppp/3p4/4P3/4P1b1/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 4", wClk: 53, bClk: 52, cpBefore: 80, cpAfter: 90, label: "Neutral" },
        { san: "Bxf3", uci: "g4f3", fenBefore: "rn1qkbnr/ppp2ppp/3p4/4P3/4P1b1/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 4", fenAfter: "rn1qkbnr/ppp2ppp/3p4/4P3/4P3/5b2/PPP2PPP/RNBQKB1R w KQkq - 0 5", wClk: 53, bClk: 48, cpBefore: 90, cpAfter: 120, label: "UnderthinkCritical" },
        { san: "Qxf3", uci: "d1f3", fenBefore: "rn1qkbnr/ppp2ppp/3p4/4P3/4P3/5b2/PPP2PPP/RNBQKB1R w KQkq - 0 5", fenAfter: "rn1qkbnr/ppp2ppp/3p4/4P3/4P3/5Q2/PPP2PPP/RNB1KB1R b KQkq - 0 5", wClk: 51, bClk: 48, cpBefore: 120, cpAfter: 130, label: "Neutral" },
        { san: "dxe5", uci: "d6e5", fenBefore: "rn1qkbnr/ppp2ppp/3p4/4P3/4P3/5Q2/PPP2PPP/RNB1KB1R b KQkq - 0 5", fenAfter: "rn1qkbnr/ppp2ppp/8/4p3/4P3/5Q2/PPP2PPP/RNB1KB1R w KQkq - 0 6", wClk: 51, bClk: 45, cpBefore: 130, cpAfter: 150, label: "Neutral" },
        { san: "Bc4", uci: "f1c4", fenBefore: "rn1qkbnr/ppp2ppp/8/4p3/4P3/5Q2/PPP2PPP/RNB1KB1R w KQkq - 0 6", fenAfter: "rn1qkbnr/ppp2ppp/8/4p3/2B1P3/5Q2/PPP2PPP/RNB1K2R b KQkq - 1 6", wClk: 49, bClk: 45, cpBefore: 150, cpAfter: 180, label: "GoodInvestment" },
        { san: "Nf6", uci: "g8f6", fenBefore: "rn1qkbnr/ppp2ppp/8/4p3/2B1P3/5Q2/PPP2PPP/RNB1K2R b KQkq - 1 6", fenAfter: "rn1qkb1r/ppp2ppp/5n2/4p3/2B1P3/5Q2/PPP2PPP/RNB1K2R w KQkq - 2 7", wClk: 49, bClk: 42, cpBefore: 180, cpAfter: 200, label: "Neutral" },
        { san: "Qb3", uci: "f3b3", fenBefore: "rn1qkb1r/ppp2ppp/5n2/4p3/2B1P3/5Q2/PPP2PPP/RNB1K2R w KQkq - 2 7", fenAfter: "rn1qkb1r/ppp2ppp/5n2/4p3/2B1P3/1Q6/PPP2PPP/RNB1K2R b KQkq - 3 7", wClk: 46, bClk: 42, cpBefore: 200, cpAfter: 250, label: "GoodInvestment" },
        { san: "Qe7", uci: "d8e7", fenBefore: "rn1qkb1r/ppp2ppp/5n2/4p3/2B1P3/1Q6/PPP2PPP/RNB1K2R b KQkq - 3 7", fenAfter: "rn2kb1r/ppp1qppp/5n2/4p3/2B1P3/1Q6/PPP2PPP/RNB1K2R w KQkq - 4 8", wClk: 46, bClk: 38, cpBefore: 250, cpAfter: 280, label: "UnderthinkCritical" },
        { san: "Nc3", uci: "b1c3", fenBefore: "rn2kb1r/ppp1qppp/5n2/4p3/2B1P3/1Q6/PPP2PPP/RNB1K2R w KQkq - 4 8", fenAfter: "rn2kb1r/ppp1qppp/5n2/4p3/2B1P3/1QN5/PPP2PPP/R1B1K2R b KQkq - 5 8", wClk: 44, bClk: 38, cpBefore: 280, cpAfter: 300, label: "Neutral" },
        { san: "c6", uci: "c7c6", fenBefore: "rn2kb1r/ppp1qppp/5n2/4p3/2B1P3/1QN5/PPP2PPP/R1B1K2R b KQkq - 5 8", fenAfter: "rn2kb1r/pp2qppp/2p2n2/4p3/2B1P3/1QN5/PPP2PPP/R1B1K2R w KQkq - 0 9", wClk: 44, bClk: 34, cpBefore: 300, cpAfter: 320, label: "WastedThink" },
        { san: "Bg5", uci: "c1g5", fenBefore: "rn2kb1r/pp2qppp/2p2n2/4p3/2B1P3/1QN5/PPP2PPP/R1B1K2R w KQkq - 0 9", fenAfter: "rn2kb1r/pp2qppp/2p2n2/4p1B1/2B1P3/1QN5/PPP2PPP/R3K2R b KQkq - 1 9", wClk: 41, bClk: 34, cpBefore: 320, cpAfter: 380, label: "GoodInvestment" },
        { san: "b5", uci: "b7b5", fenBefore: "rn2kb1r/pp2qppp/2p2n2/4p1B1/2B1P3/1QN5/PPP2PPP/R3K2R b KQkq - 1 9", fenAfter: "rn2kb1r/p3qppp/2p2n2/1p2p1B1/2B1P3/1QN5/PPP2PPP/R3K2R w KQkq - 0 10", wClk: 41, bClk: 28, cpBefore: 380, cpAfter: 500, label: "SnapBlunder" },
        { san: "Nxb5", uci: "c3b5", fenBefore: "rn2kb1r/p3qppp/2p2n2/1p2p1B1/2B1P3/1QN5/PPP2PPP/R3K2R w KQkq - 0 10", fenAfter: "rn2kb1r/p3qppp/2p2n2/1N2p1B1/2B1P3/1Q6/PPP2PPP/R3K2R b KQkq - 0 10", wClk: 38, bClk: 28, cpBefore: 500, cpAfter: 600, label: "GoodInvestment" },
        { san: "cxb5", uci: "c6b5", fenBefore: "rn2kb1r/p3qppp/2p2n2/1N2p1B1/2B1P3/1Q6/PPP2PPP/R3K2R b KQkq - 0 10", fenAfter: "rn2kb1r/p3qppp/5n2/1p2p1B1/2B1P3/1Q6/PPP2PPP/R3K2R w KQkq - 0 11", wClk: 38, bClk: 24, cpBefore: 600, cpAfter: 700, label: "UnderthinkCritical" },
        { san: "Bxb5+", uci: "c4b5", fenBefore: "rn2kb1r/p3qppp/5n2/1p2p1B1/2B1P3/1Q6/PPP2PPP/R3K2R w KQkq - 0 11", fenAfter: "rn2kb1r/p3qppp/5n2/1B2p1B1/4P3/1Q6/PPP2PPP/R3K2R b KQkq - 0 11", wClk: 35, bClk: 24, cpBefore: 700, cpAfter: 800, label: "GoodInvestment" },
        { san: "Nbd7", uci: "b8d7", fenBefore: "rn2kb1r/p3qppp/5n2/1B2p1B1/4P3/1Q6/PPP2PPP/R3K2R b KQkq - 0 11", fenAfter: "r3kb1r/p2nqppp/5n2/1B2p1B1/4P3/1Q6/PPP2PPP/R3K2R w KQkq - 1 12", wClk: 35, bClk: 19, cpBefore: 800, cpAfter: 900, label: "TimeBlunder" },
        { san: "O-O-O", uci: "e1c1", fenBefore: "r3kb1r/p2nqppp/5n2/1B2p1B1/4P3/1Q6/PPP2PPP/R3K2R w KQkq - 1 12", fenAfter: "r3kb1r/p2nqppp/5n2/1B2p1B1/4P3/1Q6/PPP2PPP/2KR3R b kq - 2 12", wClk: 32, bClk: 19, cpBefore: 900, cpAfter: 1000, label: "GoodInvestment" },
        { san: "Rd8", uci: "a8d8", fenBefore: "r3kb1r/p2nqppp/5n2/1B2p1B1/4P3/1Q6/PPP2PPP/2KR3R b kq - 2 12", fenAfter: "3rkb1r/p2nqppp/5n2/1B2p1B1/4P3/1Q6/PPP2PPP/2KR3R w k - 3 13", wClk: 32, bClk: 14, cpBefore: 1000, cpAfter: 1200, label: "TimeBlunder" },
        { san: "Rxd7", uci: "d1d7", fenBefore: "3rkb1r/p2nqppp/5n2/1B2p1B1/4P3/1Q6/PPP2PPP/2KR3R w k - 3 13", fenAfter: "3rkb1r/p2Rqppp/5n2/1B2p1B1/4P3/1Q6/PPP2PPP/2K4R b k - 0 13", wClk: 28, bClk: 14, cpBefore: 1200, cpAfter: 1500, label: "GoodInvestment" },
        { san: "Rxd7", uci: "d8d7", fenBefore: "3rkb1r/p2Rqppp/5n2/1B2p1B1/4P3/1Q6/PPP2PPP/2K4R b k - 0 13", fenAfter: "4kb1r/p2rqppp/5n2/1B2p1B1/4P3/1Q6/PPP2PPP/2K4R w k - 0 14", wClk: 28, bClk: 10, cpBefore: 1500, cpAfter: 1800, label: "TimeBlunder" },
        { san: "Rd1", uci: "h1d1", fenBefore: "4kb1r/p2rqppp/5n2/1B2p1B1/4P3/1Q6/PPP2PPP/2K4R w k - 0 14", fenAfter: "4kb1r/p2rqppp/5n2/1B2p1B1/4P3/1Q6/PPP2PPP/2KR4 b k - 1 14", wClk: 25, bClk: 10, cpBefore: 1800, cpAfter: 2000, label: "GoodInvestment" },
        { san: "Qe6", uci: "e7e6", fenBefore: "4kb1r/p2rqppp/5n2/1B2p1B1/4P3/1Q6/PPP2PPP/2KR4 b k - 1 14", fenAfter: "4kb1r/p2r1ppp/4qn2/1B2p1B1/4P3/1Q6/PPP2PPP/2KR4 w k - 2 15", wClk: 25, bClk: 6, cpBefore: 2000, cpAfter: 2500, label: "TimeBlunder" },
        { san: "Bxd7+", uci: "b5d7", fenBefore: "4kb1r/p2r1ppp/4qn2/1B2p1B1/4P3/1Q6/PPP2PPP/2KR4 w k - 2 15", fenAfter: "4kb1r/p2B1ppp/4qn2/4p1B1/4P3/1Q6/PPP2PPP/2KR4 b k - 0 15", wClk: 22, bClk: 6, cpBefore: 2500, cpAfter: 3000, label: "GoodInvestment" },
        { san: "Nxd7", uci: "f6d7", fenBefore: "4kb1r/p2B1ppp/4qn2/4p1B1/4P3/1Q6/PPP2PPP/2KR4 b k - 0 15", fenAfter: "4kb1r/p2n1ppp/4q3/4p1B1/4P3/1Q6/PPP2PPP/2KR4 w k - 0 16", wClk: 22, bClk: 3, cpBefore: 3000, cpAfter: 5000, label: "TimeBlunder" },
        { san: "Qb8+", uci: "b3b8", fenBefore: "4kb1r/p2n1ppp/4q3/4p1B1/4P3/1Q6/PPP2PPP/2KR4 w k - 0 16", fenAfter: "1Q2kb1r/p2n1ppp/4q3/4p1B1/4P3/8/PPP2PPP/2KR4 b k - 1 16", wClk: 19, bClk: 3, cpBefore: 5000, cpAfter: 9999, label: "GoodInvestment" },
        { san: "Nxb8", uci: "d7b8", fenBefore: "1Q2kb1r/p2n1ppp/4q3/4p1B1/4P3/8/PPP2PPP/2KR4 b k - 1 16", fenAfter: "1n2kb1r/p4ppp/4q3/4p1B1/4P3/8/PPP2PPP/2KR4 w k - 0 17", wClk: 19, bClk: 1, cpBefore: 9999, cpAfter: 9999, label: "TimeBlunder" },
        { san: "Rd8#", uci: "d1d8", fenBefore: "1n2kb1r/p4ppp/4q3/4p1B1/4P3/8/PPP2PPP/2KR4 w k - 0 17", fenAfter: "1n1Rkb1r/p4ppp/4q3/4p1B1/4P3/8/PPP2PPP/2K5 b k - 1 17", wClk: 16, bClk: 1, cpBefore: 9999, cpAfter: 9999, label: "GoodInvestment" },
    ];

    const plies: PlyAnalysis[] = moves.map((m, idx) => {
        const mover = idx % 2 === 0 ? "White" : "Black";
        const prevClock = idx === 0 ? 60 : (mover === "White" ? moves[idx - 2]?.wClk ?? 60 : moves[idx - 1]?.bClk ?? 60);
        const currClock = mover === "White" ? m.wClk : m.bClk;
        const thinkTime = prevClock - currClock;

        // Calculate time equity: positive when white has more time
        const tauWhite = Math.round((m.wClk - m.bClk) * 5); // ~5cp per second advantage

        const ply: PlyRecord = {
            ply_index: idx + 1,
            san: m.san,
            uci: m.uci,
            mover: mover as "White" | "Black",
            fen_before: m.fenBefore,
            fen_after: m.fenAfter,
            clock_after_secs: currClock,
            clock_before_secs: prevClock,
            think_time_secs: thinkTime,
        };

        const label = makeLabel(
            m.label,
            m.label === "Neutral" ? 0.1 : m.label === "GoodInvestment" ? 0.3 : m.label === "SnapBlunder" ? 0.9 : 0.6,
            getLabelTitle(m.label),
            getLabelExplanation(m.label, m.san, thinkTime)
        );

        return {
            ply,
            engine_before: {
                depth: 14,
                nodes: 1000000,
                nps: 1000000,
                lines: [],
                best_cp_white: m.cpBefore,
                played_cp_white: m.cpAfter,
            },
            metrics: makeMetrics(m.cpBefore, m.cpAfter, tauWhite, (m.cpAfter - m.cpBefore) / 100),
            label,
        };
    });

    return {
        meta: {
            white: "Paul Morphy",
            black: "Duke of Brunswick & Count Isouard",
            result: "1-0",
            platform: "Unknown" as const,
            time_control: { base_secs: 60, increment_secs: 0 },
        },
        plies,
        summary: {
            total_plies: plies.length,
            labels_count: {},
            time_trouble_moves: 5,
            panic_moves: 2,
            blunders_in_time_trouble: 1,
            phase_time_share: { opening: 0.25, middlegame: 0.55, endgame: 0.20 },
            phase_time_share_delta_vs_15_70_15: { opening: 0.10, middlegame: -0.15, endgame: 0.05 },
            phase_avg_think_time_secs: { opening: 2.5, middlegame: 3.2, endgame: 1.8 },
        },
    };
}

function getLabelTitle(kind: string): string {
    switch (kind) {
        case "OverthinkSimple": return "Overthink on Simple Position";
        case "UnderthinkCritical": return "Rushed Critical Decision";
        case "WastedThink": return "Time Wasted";
        case "GoodInvestment": return "Time Well Spent";
        case "SnapBlunder": return "Snap Move Blunder";
        case "TimeBlunder": return "Time Pressure Mistake";
        case "Neutral": return "Normal Move";
        default: return kind;
    }
}

function getLabelExplanation(kind: string, san: string, thinkTime: number): string {
    switch (kind) {
        case "OverthinkSimple": return `Spent ${thinkTime}s on ${san} in a straightforward position.`;
        case "UnderthinkCritical": return `Only ${thinkTime}s on ${san} - this position deserved more thought.`;
        case "WastedThink": return `${thinkTime}s on ${san} didn't improve the position.`;
        case "GoodInvestment": return `${thinkTime}s well spent on ${san} - found a strong continuation.`;
        case "SnapBlunder": return `Instant ${san} was a mistake - take a breath next time.`;
        case "TimeBlunder": return `With only seconds left, ${san} was a time-pressure mistake.`;
        case "Neutral": return `${san} was a reasonable move in normal time.`;
        default: return `Move ${san} played in ${thinkTime}s.`;
    }
}

// The pre-computed mock analysis
export const MOCK_OPERA_HOUSE_ANALYSIS: GameAnalysis = createOperaHouseAnalysis();
