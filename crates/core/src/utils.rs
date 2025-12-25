use crate::model::Color;

pub fn mover_cp(cp_white: i32, mover: Color) -> i32 {
    match mover {
        Color::White => cp_white,
        Color::Black => -cp_white,
    }
}

pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}
