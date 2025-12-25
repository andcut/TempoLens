use crate::model::{Color, PlyRecord, SourcePlatform, TimeControl};

#[derive(Debug, Clone, Copy)]
pub struct ClockPolicy {
    pub increment_applied_after_move: bool,
}

impl ClockPolicy {
    pub fn for_platform(platform: SourcePlatform) -> Self {
        match platform {
            SourcePlatform::Lichess | SourcePlatform::ChessCom => Self {
                increment_applied_after_move: true,
            },
            SourcePlatform::Unknown => Self {
                increment_applied_after_move: true,
            },
        }
    }
}

pub fn derive_clock_before_and_think_times(
    plies: &mut [PlyRecord],
    tc: Option<TimeControl>,
    platform: SourcePlatform,
) {
    let inc = tc.as_ref().map(|t| t.increment_secs as f32).unwrap_or(0.0);
    let base = tc.as_ref().map(|t| t.base_secs as f32);
    let mut policy = ClockPolicy::for_platform(platform);
    if platform == SourcePlatform::Unknown && inc > 0.0 {
        policy = infer_policy(plies, base, inc, policy);
    }

    let mut last_white_after: Option<f32> = None;
    let mut last_black_after: Option<f32> = None;

    for ply in plies.iter_mut() {
        let t_before = match ply.mover {
            Color::White => last_white_after.or(base),
            Color::Black => last_black_after.or(base),
        };

        ply.clock_before_secs = t_before;

        if let (Some(t_before), Some(t_after)) = (t_before, ply.clock_after_secs) {
            let mut spent = if policy.increment_applied_after_move {
                (t_before + inc) - t_after
            } else {
                t_before - t_after
            };
            let max_spent = t_before + inc;
            if spent < 0.0 {
                spent = 0.0;
            }
            if spent > max_spent {
                spent = max_spent;
            }
            ply.think_time_secs = Some(spent);
        }

        match ply.mover {
            Color::White => last_white_after = ply.clock_after_secs,
            Color::Black => last_black_after = ply.clock_after_secs,
        }
    }
}

fn infer_policy(
    plies: &[PlyRecord],
    base: Option<f32>,
    inc: f32,
    default_policy: ClockPolicy,
) -> ClockPolicy {
    let applied_penalty = policy_penalty(plies, base, inc, true);
    let raw_penalty = policy_penalty(plies, base, inc, false);

    if applied_penalty < raw_penalty {
        ClockPolicy {
            increment_applied_after_move: true,
        }
    } else if raw_penalty < applied_penalty {
        ClockPolicy {
            increment_applied_after_move: false,
        }
    } else {
        default_policy
    }
}

fn policy_penalty(plies: &[PlyRecord], base: Option<f32>, inc: f32, applied: bool) -> u32 {
    let mut last_white_after: Option<f32> = None;
    let mut last_black_after: Option<f32> = None;
    let mut penalty = 0;
    let tolerance = 0.5;

    for ply in plies.iter() {
        let t_before = match ply.mover {
            Color::White => last_white_after.or(base),
            Color::Black => last_black_after.or(base),
        };

        if let (Some(t_before), Some(t_after)) = (t_before, ply.clock_after_secs) {
            let spent = if applied {
                (t_before + inc) - t_after
            } else {
                t_before - t_after
            };

            let max_spent = if applied { t_before + inc } else { t_before };
            if spent < -tolerance || spent > max_spent + tolerance {
                penalty += 1;
            }
        }

        match ply.mover {
            Color::White => last_white_after = ply.clock_after_secs,
            Color::Black => last_black_after = ply.clock_after_secs,
        }
    }

    penalty
}
