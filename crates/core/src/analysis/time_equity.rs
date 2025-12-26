use crate::model::{Color, PHASE_MIDDLEGAME_END_PLY, PHASE_OPENING_END_PLY};
use crate::utils::sigmoid;

pub fn phase_multiplier(ply_index: u32) -> f32 {
    if ply_index < PHASE_OPENING_END_PLY {
        0.85
    } else if ply_index < PHASE_MIDDLEGAME_END_PLY {
        1.0
    } else {
        1.15
    }
}

#[allow(clippy::too_many_arguments)]
pub fn time_equity_white_cp(
    alpha: f32,
    beta: f32,
    pressure_pivot: f32,
    pressure_scale: f32,
    pressure_boost: f32,
    t_white: f32,
    t_black: f32,
    ply_index: u32,
) -> i32 {
    let t_total = t_white + t_black;
    let v = alpha / (t_total + beta);
    let pressure = time_pressure_multiplier(t_total, pressure_pivot, pressure_scale, pressure_boost);
    let tau_pawns = v * pressure * (t_white - t_black) * phase_multiplier(ply_index);
    (tau_pawns * 100.0).round() as i32
}

fn time_pressure_multiplier(total_secs: f32, pivot: f32, scale: f32, boost: f32) -> f32 {
    if boost <= 0.0 {
        return 1.0;
    }
    let scale = if scale.abs() < f32::EPSILON { 1.0 } else { scale };
    let z = (total_secs - pivot) / scale;
    let sigmoid = 1.0 / (1.0 + (-z).exp());
    1.0 + boost * (1.0 - sigmoid)
}

pub fn win_prob_from_cp(k: f32, cp_white: i32) -> f32 {
    let x = k * (cp_white as f32 / 100.0);
    sigmoid(x)
}

pub fn mover_prob(p_white: f32, mover: Color) -> f32 {
    match mover {
        Color::White => p_white,
        Color::Black => 1.0 - p_white,
    }
}
