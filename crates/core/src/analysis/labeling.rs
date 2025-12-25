use crate::model::{Label, LabelKind};

#[derive(Debug, Clone)]
pub struct LabelConfig {
    pub overthink_ratio: f32,
    pub underthink_ratio: f32,
    pub min_snap_secs: f32,
    pub time_trouble_secs: f32,
    pub panic_secs: f32,
    pub max_simple_complexity: i32,
    pub critical_complexity: i32,
    pub big_punish: i32,
    pub snap_punish: i32,
    pub time_blunder_drop: f32,
}

impl Default for LabelConfig {
    fn default() -> Self {
        Self {
            overthink_ratio: 0.25,
            underthink_ratio: 0.03,
            min_snap_secs: 1.0,
            time_trouble_secs: 10.0,
            panic_secs: 5.0,
            max_simple_complexity: 40,
            critical_complexity: 120,
            big_punish: 150,
            snap_punish: 250,
            time_blunder_drop: -0.10,
        }
    }
}

pub fn label_move(
    cfg: &LabelConfig,
    think_time: Option<f32>,
    t_rem_before: Option<f32>,
    punish_cp_mover: Option<i32>,
    complexity_cp_mover: Option<i32>,
    dp_practical_mover: f32,
) -> Label {
    let spent = think_time.unwrap_or(0.0);
    let t_rem = t_rem_before.unwrap_or(999.0);

    let punish = punish_cp_mover.unwrap_or(0);
    let complex = complexity_cp_mover.unwrap_or(punish);

    let in_time_trouble = t_rem_before
        .map(|t| t <= cfg.time_trouble_secs)
        .unwrap_or(false);
    let in_panic = t_rem_before.map(|t| t <= cfg.panic_secs).unwrap_or(false);

    let overthink = spent > (cfg.overthink_ratio * t_rem)
        && complex < cfg.max_simple_complexity
        && dp_practical_mover < 0.0;

    let underthink = spent < (cfg.underthink_ratio * t_rem).min(cfg.min_snap_secs)
        && complex > cfg.critical_complexity
        && punish > cfg.big_punish;

    let wasted = spent > (cfg.overthink_ratio * t_rem) && punish > cfg.big_punish;
    let snap = spent < cfg.min_snap_secs && punish > cfg.snap_punish;
    let panic_blunder = in_panic && punish > cfg.big_punish;
    let time_blunder = in_time_trouble
        && dp_practical_mover < cfg.time_blunder_drop
        && punish < cfg.max_simple_complexity;

    let (kind, title) = if snap {
        (LabelKind::SnapBlunder, "Snap blunder")
    } else if panic_blunder {
        (LabelKind::PanicBlunder, "Panic blunder")
    } else if time_blunder {
        (LabelKind::TimeBlunder, "Time blunder")
    } else if wasted {
        (LabelKind::WastedThink, "Wasted think")
    } else if overthink {
        (LabelKind::OverthinkSimple, "Overthinking a simple position")
    } else if underthink {
        (LabelKind::UnderthinkCritical, "Underthinking a critical moment")
    } else if dp_practical_mover > 0.05 && complex > cfg.critical_complexity {
        (LabelKind::GoodInvestment, "Good investment")
    } else if in_time_trouble {
        (LabelKind::TimeTrouble, "Time trouble")
    } else {
        (LabelKind::Neutral, "Neutral")
    };

    let severity = if snap || panic_blunder {
        0.9
    } else if time_blunder {
        0.7
    } else if wasted || underthink {
        0.6
    } else if overthink {
        0.5
    } else if in_time_trouble {
        0.4
    } else {
        0.3
    };

    let time_hint = if let Some(t) = t_rem_before {
        format!("{:.1}s remaining", t)
    } else {
        "remaining unknown".to_string()
    };

    let mut tips = vec![
        "In blitz, spend time where the position is knife-edge; play instantly where it's not."
            .to_string(),
    ];
    if in_time_trouble {
        tips.push(format!(
            "Try to keep at least {:.0}s before critical moments.",
            cfg.time_trouble_secs
        ));
    }

    Label {
        kind,
        severity,
        title: title.to_string(),
        explanation: format!(
            "Spent {:.1}s, {}, complexity ~{}cp, practical Δp={:.3}",
            spent, time_hint, complex, dp_practical_mover
        ),
        tips,
    }
}
