use crate::model::Color;
use crate::utils::sigmoid;

pub fn phase_multiplier(ply_index: u32) -> f32 {
    if ply_index < 20 {
        0.85
    } else if ply_index < 60 {
        1.0
    } else {
        1.15
    }
}

pub fn time_equity_white_cp(
    alpha: f32,
    beta: f32,
    t_white: f32,
    t_black: f32,
    ply_index: u32,
) -> i32 {
    let t_total = t_white + t_black;
    let v = alpha / (t_total + beta);
    let tau_pawns = v * (t_white - t_black) * phase_multiplier(ply_index);
    (tau_pawns * 100.0).round() as i32
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
