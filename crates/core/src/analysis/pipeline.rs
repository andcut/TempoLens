use anyhow::{anyhow, Result};
use serde_json::json;
use std::collections::HashMap;

use crate::analysis::eval::{fill_engine_metrics, normalize_summary_for_white};
use crate::analysis::labeling::{label_move, LabelConfig};
use crate::analysis::position::build_ply_records_with_fens;
use crate::analysis::time_equity::{mover_prob, time_equity_white_cp, win_prob_from_cp};
use crate::clocks::derive_clock_before_and_think_times;
use crate::engine::uci::UciEngine;
use crate::model::{Color, EngineSummary, GameAnalysis, GameMeta, MoveMetrics, PlyAnalysis};
use crate::pgn::{detect_platform, parse_single_game, parse_time_control_header};

#[derive(Debug, Clone)]
pub struct AnalysisConfig {
    pub engine_path: String,
    pub multipv: u8,
    pub depth: u16,
    pub movetime_ms: Option<u64>,
    pub threads: Option<u32>,
    pub hash_mb: Option<u32>,
    pub fallback_time_control: Option<crate::model::TimeControl>,
    pub alpha: f32,
    pub beta: f32,
    pub k_sigmoid: f32,
    pub label_config: LabelConfig,
}

pub async fn analyze_pgn(pgn: &str, cfg: AnalysisConfig) -> Result<GameAnalysis> {
    let parsed = parse_single_game(pgn)?;
    let platform = detect_platform(&parsed.headers);
    let time_control =
        parse_time_control_header(&parsed.headers).or_else(|| cfg.fallback_time_control.clone());

    let mut plies = build_ply_records_with_fens(&parsed)?;
    derive_clock_before_and_think_times(&mut plies, time_control.clone(), platform);

    let meta = GameMeta {
        event: parsed.headers.get("Event").cloned(),
        site: parsed.headers.get("Site").cloned(),
        date: parsed.headers.get("Date").cloned(),
        round: parsed.headers.get("Round").cloned(),
        white: parsed.headers.get("White").cloned(),
        black: parsed.headers.get("Black").cloned(),
        result: parsed.headers.get("Result").cloned(),
        time_control: time_control.clone(),
        platform,
        headers: parsed.headers.clone(),
    };

    let mut engine = UciEngine::start(&cfg.engine_path).await?;
    if let Some(threads) = cfg.threads {
        engine.set_option("Threads", &threads.to_string()).await?;
    }
    if let Some(hash_mb) = cfg.hash_mb {
        engine.set_option("Hash", &hash_mb.to_string()).await?;
    }
    engine
        .set_option("MultiPV", &cfg.multipv.to_string())
        .await?;
    engine.new_game().await?;

    let mut cache: HashMap<String, EngineSummary> = HashMap::new();
    let mut summaries: Vec<EngineSummary> = Vec::with_capacity(plies.len());

    for ply in plies.iter() {
        let mut summary = analyze_position(
            &mut engine,
            &mut cache,
            &ply.fen_before,
            cfg.depth,
            cfg.movetime_ms,
            cfg.multipv,
        )
        .await?;

        normalize_summary_for_white(&mut summary, ply.mover);

        let played_cp = summary
            .lines
            .iter()
            .find(|l| l.uci == ply.uci)
            .map(|l| l.cp_white);

        summary.played_cp_white = if let Some(cp) = played_cp {
            Some(cp)
        } else {
            let mut search_summary = analyze_position_searchmove(
                &mut engine,
                &ply.fen_before,
                &ply.uci,
                cfg.depth,
                cfg.movetime_ms,
            )
            .await?;
            normalize_summary_for_white(&mut search_summary, ply.mover);
            search_summary.lines.first().map(|l| l.cp_white)
        };

        fill_engine_metrics(&mut summary, ply.mover);
        summaries.push(summary);
    }

    let mut last_after_summary: Option<EngineSummary> = None;
    if let Some(last_ply) = plies.last() {
        if !last_ply.fen_after.is_empty() {
            let summary = analyze_position(
                &mut engine,
                &mut cache,
                &last_ply.fen_after,
                cfg.depth,
                cfg.movetime_ms,
                cfg.multipv,
            )
            .await?;
            let mut summary = summary;
            let side_to_move = match last_ply.mover {
                Color::White => Color::Black,
                Color::Black => Color::White,
            };
            normalize_summary_for_white(&mut summary, side_to_move);
            if let Some(best) = summary.lines.first().map(|l| l.cp_white) {
                summary.best_cp_white = Some(best);
            }
            last_after_summary = Some(summary);
        }
    }

    let clock_states = derive_clock_states(&plies, time_control);

    let mut analyses: Vec<PlyAnalysis> = Vec::with_capacity(plies.len());
    for (idx, ply) in plies.into_iter().enumerate() {
        let summary = summaries
            .get(idx)
            .cloned()
            .ok_or_else(|| anyhow!("Missing engine summary for ply {}", idx + 1))?;

        let cp_eval_before = summary.best_cp_white.unwrap_or(0);
        let cp_eval_after = summaries
            .get(idx + 1)
            .and_then(|s| s.best_cp_white)
            .or_else(|| {
                if idx + 1 == summaries.len() {
                    last_after_summary.as_ref().and_then(|s| s.best_cp_white)
                } else {
                    None
                }
            })
            .unwrap_or(cp_eval_before);

        let clocks = clock_states.get(idx).cloned();
        let tau_before = clocks
            .as_ref()
            .and_then(|c| match (c.before_white, c.before_black) {
                (Some(w), Some(b)) => Some(time_equity_white_cp(cfg.alpha, cfg.beta, w, b, ply.ply_index)),
                _ => None,
            })
            .unwrap_or(0);
        let tau_after = clocks
            .as_ref()
            .and_then(|c| match (c.after_white, c.after_black) {
                (Some(w), Some(b)) => Some(time_equity_white_cp(cfg.alpha, cfg.beta, w, b, ply.ply_index)),
                _ => None,
            })
            .unwrap_or(0);

        let cp_practical_before = cp_eval_before + tau_before;
        let cp_practical_after = cp_eval_after + tau_after;

        let p_eval_before = win_prob_from_cp(cfg.k_sigmoid, cp_eval_before);
        let p_eval_after = win_prob_from_cp(cfg.k_sigmoid, cp_eval_after);
        let p_practical_before = win_prob_from_cp(cfg.k_sigmoid, cp_practical_before);
        let p_practical_after = win_prob_from_cp(cfg.k_sigmoid, cp_practical_after);

        let dp_eval_mover = mover_prob(p_eval_after, ply.mover) - mover_prob(p_eval_before, ply.mover);
        let dp_practical_mover =
            mover_prob(p_practical_after, ply.mover) - mover_prob(p_practical_before, ply.mover);

        let metrics = MoveMetrics {
            tau_white_cp: tau_before,
            cp_eval_before,
            cp_eval_after,
            cp_practical_before,
            cp_practical_after,
            p_eval_before,
            p_eval_after,
            p_practical_before,
            p_practical_after,
            dp_eval_mover,
            dp_practical_mover,
        };

        let label = label_move(
            &cfg.label_config,
            ply.think_time_secs,
            ply.clock_before_secs,
            summary.punish_cp_mover,
            summary.spread_k_cp_mover,
            dp_practical_mover,
        );

        analyses.push(PlyAnalysis {
            ply,
            engine_before: summary,
            metrics,
            label,
        });
    }

    let summary = build_summary(&analyses);

    Ok(GameAnalysis {
        meta,
        plies: analyses,
        summary,
    })
}

async fn analyze_position(
    engine: &mut UciEngine,
    cache: &mut HashMap<String, EngineSummary>,
    fen: &str,
    depth: u16,
    movetime_ms: Option<u64>,
    multipv: u8,
) -> Result<EngineSummary> {
    if let Some(summary) = cache.get(fen) {
        return Ok(summary.clone());
    }

    engine.position_fen(fen).await?;
    let summary = engine
        .go_multipv(depth, movetime_ms, multipv, None)
        .await?;
    cache.insert(fen.to_string(), summary.clone());
    Ok(summary)
}

async fn analyze_position_searchmove(
    engine: &mut UciEngine,
    fen: &str,
    move_uci: &str,
    depth: u16,
    movetime_ms: Option<u64>,
) -> Result<EngineSummary> {
    engine.position_fen(fen).await?;
    let summary = engine
        .go_multipv(depth, movetime_ms, 1, Some(move_uci))
        .await?;
    Ok(summary)
}

#[derive(Clone, Debug)]
struct ClockState {
    before_white: Option<f32>,
    before_black: Option<f32>,
    after_white: Option<f32>,
    after_black: Option<f32>,
}

fn derive_clock_states(
    plies: &[crate::model::PlyRecord],
    tc: Option<crate::model::TimeControl>,
) -> Vec<ClockState> {
    let mut out = Vec::with_capacity(plies.len());
    let mut last_white = tc.as_ref().map(|t| t.base_secs as f32);
    let mut last_black = tc.as_ref().map(|t| t.base_secs as f32);

    for ply in plies.iter() {
        let mut before_white = last_white;
        let mut before_black = last_black;

        match ply.mover {
            Color::White => {
                if let Some(b) = ply.clock_before_secs {
                    before_white = Some(b);
                }
            }
            Color::Black => {
                if let Some(b) = ply.clock_before_secs {
                    before_black = Some(b);
                }
            }
        }

        let mut after_white = before_white;
        let mut after_black = before_black;
        match ply.mover {
            Color::White => {
                if let Some(a) = ply.clock_after_secs {
                    after_white = Some(a);
                }
                last_white = after_white;
            }
            Color::Black => {
                if let Some(a) = ply.clock_after_secs {
                    after_black = Some(a);
                }
                last_black = after_black;
            }
        }

        out.push(ClockState {
            before_white,
            before_black,
            after_white,
            after_black,
        });
    }

    out
}

fn build_summary(analyses: &[PlyAnalysis]) -> serde_json::Value {
    let mut label_counts: HashMap<String, u32> = HashMap::new();
    let mut think_times: Vec<f32> = Vec::new();
    let mut punish: Vec<i32> = Vec::new();
    let mut dp_practical: Vec<f32> = Vec::new();

    for ply in analyses {
        let key = format!("{:?}", ply.label.kind);
        *label_counts.entry(key).or_insert(0) += 1;

        if let Some(t) = ply.ply.think_time_secs {
            think_times.push(t);
        }
        if let Some(p) = ply.engine_before.punish_cp_mover {
            punish.push(p);
        }
        dp_practical.push(ply.metrics.dp_practical_mover);
    }

    let avg_think_time = average_f32(&think_times);
    let avg_punish = average_i32(&punish);
    let avg_dp_practical = average_f32(&dp_practical);

    json!({
        "total_plies": analyses.len(),
        "labels_count": label_counts,
        "avg_think_time_secs": avg_think_time,
        "avg_punish_cp_mover": avg_punish,
        "avg_dp_practical_mover": avg_dp_practical,
    })
}

fn average_f32(values: &[f32]) -> Option<f32> {
    if values.is_empty() {
        return None;
    }
    let sum: f32 = values.iter().sum();
    Some(sum / values.len() as f32)
}

fn average_i32(values: &[i32]) -> Option<f32> {
    if values.is_empty() {
        return None;
    }
    let sum: i32 = values.iter().sum();
    Some(sum as f32 / values.len() as f32)
}
