use crate::model::{Color, EngineSummary};
use crate::utils::mover_cp;

pub fn normalize_cp_white(cp_raw: i32, side_to_move: Color) -> i32 {
    match side_to_move {
        Color::White => cp_raw,
        Color::Black => -cp_raw,
    }
}

pub fn normalize_summary_for_white(summary: &mut EngineSummary, side_to_move: Color) {
    for line in summary.lines.iter_mut() {
        line.cp_white = normalize_cp_white(line.cp_white, side_to_move);
        if let Some(mate) = line.mate.as_mut() {
            *mate = normalize_cp_white(*mate, side_to_move);
        }
    }

    if let Some(cp) = summary.played_cp_white {
        summary.played_cp_white = Some(normalize_cp_white(cp, side_to_move));
    }

    if let Some(cp) = summary.best_cp_white {
        summary.best_cp_white = Some(normalize_cp_white(cp, side_to_move));
    }
}

pub fn fill_engine_metrics(summary: &mut EngineSummary, mover: Color) {
    if let Some(best) = summary.lines.first().map(|l| l.cp_white) {
        summary.best_cp_white = Some(best);
    }

    if summary.lines.len() >= 2 {
        let best = summary.lines[0].cp_white;
        let second = summary.lines[1].cp_white;
        let kth = summary.lines[summary.lines.len() - 1].cp_white;
        let spread = mover_cp(best, mover) - mover_cp(kth, mover);
        summary.spread_k_cp_mover = Some(spread);

        let gap12 = mover_cp(best, mover) - mover_cp(second, mover);
        summary.gap_12_cp_mover = Some(gap12);
    }

    if let (Some(best), Some(played)) = (summary.best_cp_white, summary.played_cp_white) {
        let punish = mover_cp(best, mover) - mover_cp(played, mover);
        summary.punish_cp_mover = Some(punish);
    }

    summary.complexity_cp_mover = combine_complexity([
        summary.punish_cp_mover,
        summary.spread_k_cp_mover,
        summary.gap_12_cp_mover,
    ]);
}

fn combine_complexity(values: [Option<i32>; 3]) -> Option<i32> {
    let mut best: Option<i32> = None;
    for value in values {
        if let Some(v) = value {
            best = Some(best.map_or(v, |b| b.max(v)));
        }
    }
    best
}
