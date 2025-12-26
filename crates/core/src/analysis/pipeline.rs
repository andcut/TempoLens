use anyhow::{anyhow, Result};
use std::collections::HashMap;

use crate::analysis::eval::{fill_engine_metrics, normalize_summary_for_white};
use crate::analysis::labeling::{label_move, LabelConfig};
use crate::analysis::position::build_ply_records_with_fens;
use crate::analysis::time_equity::{mover_prob, time_equity_white_cp, win_prob_from_cp};
use crate::clocks::derive_clock_before_and_think_times;
use crate::engine::uci::UciEngine;
use crate::model::{
    Color, EngineSummary, GameAnalysis, GameMeta, GameSummary, MoveMetrics, PhaseAverages,
    PhaseTimeShare, PhaseTimeShareDelta, PlyAnalysis, PHASE_MIDDLEGAME_END_PLY,
    PHASE_OPENING_END_PLY,
};
use crate::pgn::{detect_platform, parse_games, parse_time_control_header};

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
    pub time_pressure_pivot: f32,
    pub time_pressure_scale: f32,
    pub time_pressure_boost: f32,
    pub k_sigmoid: f32,
    pub label_config: LabelConfig,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            engine_path: String::new(),
            multipv: 4,
            depth: 14,
            movetime_ms: None,
            threads: None,
            hash_mb: None,
            fallback_time_control: None,
            alpha: 2.0,
            beta: 10.0,
            time_pressure_pivot: 30.0,
            time_pressure_scale: 8.0,
            time_pressure_boost: 3.0,
            k_sigmoid: 1.2,
            label_config: LabelConfig::default(),
        }
    }
}

pub async fn analyze_pgn(pgn: &str, cfg: AnalysisConfig) -> Result<GameAnalysis> {
    let games = parse_games(pgn)?;
    if games.len() != 1 {
        return Err(anyhow!(
            "Expected exactly one PGN game, found {}.",
            games.len()
        ));
    }

    let mut engine = start_engine(&cfg).await?;
    engine.new_game().await?;
    let result = analyze_parsed_game(games.into_iter().next().unwrap(), &cfg, &mut engine).await;
    let shutdown_result = engine.shutdown().await;
    match (result, shutdown_result) {
        (Ok(analysis), Ok(())) => Ok(analysis),
        (Err(err), _) => Err(err),
        (Ok(_), Err(err)) => Err(anyhow!(err)),
    }
}

pub async fn analyze_pgns(pgn: &str, cfg: AnalysisConfig) -> Result<Vec<GameAnalysis>> {
    let games = parse_games(pgn)?;
    if games.is_empty() {
        return Err(anyhow!("No PGN games found in input"));
    }

    let mut engine = start_engine(&cfg).await?;
    let mut out = Vec::with_capacity(games.len());
    for game in games {
        engine.new_game().await?;
        out.push(analyze_parsed_game(game, &cfg, &mut engine).await?);
    }
    let shutdown_result = engine.shutdown().await;
    match shutdown_result {
        Ok(()) => Ok(out),
        Err(err) => Err(anyhow!(err)),
    }
}

async fn start_engine(cfg: &AnalysisConfig) -> Result<UciEngine> {
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
    Ok(engine)
}

async fn analyze_parsed_game(
    parsed: crate::pgn::ParsedGame,
    cfg: &AnalysisConfig,
    engine: &mut UciEngine,
) -> Result<GameAnalysis> {
    let platform = detect_platform(&parsed.headers);
    let time_control =
        parse_time_control_header(&parsed.headers).or_else(|| cfg.fallback_time_control.clone());

    let mut plies = build_ply_records_with_fens(&parsed)?;
    derive_clock_before_and_think_times(&mut plies, time_control.clone(), platform);
    let meta = build_meta(&parsed, time_control.clone(), platform);
    let (summaries, last_after_summary) = analyze_engine_summaries(engine, &plies, cfg).await?;
    let clock_states = derive_clock_states(&plies, time_control);
    let analyses = build_ply_analyses(
        plies,
        &summaries,
        last_after_summary.as_ref(),
        &clock_states,
        cfg,
    )?;
    let summary = build_summary(&analyses, &cfg.label_config);

    Ok(GameAnalysis {
        meta,
        plies: analyses,
        summary,
    })
}

fn build_meta(
    parsed: &crate::pgn::ParsedGame,
    time_control: Option<crate::model::TimeControl>,
    platform: crate::model::SourcePlatform,
) -> GameMeta {
    GameMeta {
        event: parsed.headers.get("Event").cloned(),
        site: parsed.headers.get("Site").cloned(),
        date: parsed.headers.get("Date").cloned(),
        round: parsed.headers.get("Round").cloned(),
        white: parsed.headers.get("White").cloned(),
        black: parsed.headers.get("Black").cloned(),
        result: parsed.headers.get("Result").cloned(),
        time_control,
        platform,
        headers: parsed.headers.clone(),
    }
}

async fn analyze_engine_summaries(
    engine: &mut UciEngine,
    plies: &[crate::model::PlyRecord],
    cfg: &AnalysisConfig,
) -> Result<(Vec<EngineSummary>, Option<EngineSummary>)> {
    let mut cache: HashMap<String, EngineSummary> = HashMap::new();
    let mut summaries: Vec<EngineSummary> = Vec::with_capacity(plies.len());

    for ply in plies.iter() {
        let mut summary = analyze_position(
            engine,
            &mut cache,
            &ply.fen_before,
            cfg.depth,
            cfg.movetime_ms,
            cfg.multipv,
        )
        .await?;

        normalize_summary_for_white(&mut summary, ply.mover);
        summary.played_cp_white = played_cp_for_ply(engine, ply, cfg, &summary).await?;

        fill_engine_metrics(&mut summary, ply.mover);
        summaries.push(summary);
    }

    let last_after_summary = analyze_last_after_summary(engine, plies, cfg, &mut cache).await?;
    Ok((summaries, last_after_summary))
}

async fn played_cp_for_ply(
    engine: &mut UciEngine,
    ply: &crate::model::PlyRecord,
    cfg: &AnalysisConfig,
    summary: &EngineSummary,
) -> Result<Option<i32>> {
    let played_cp = summary
        .lines
        .iter()
        .find(|l| l.uci == ply.uci)
        .map(|l| l.cp_white);

    if let Some(cp) = played_cp {
        return Ok(Some(cp));
    }

    let mut search_summary = analyze_position_searchmove(
        engine,
        &ply.fen_before,
        &ply.uci,
        cfg.depth,
        cfg.movetime_ms,
    )
    .await?;
    normalize_summary_for_white(&mut search_summary, ply.mover);
    Ok(search_summary.lines.first().map(|l| l.cp_white))
}

async fn analyze_last_after_summary(
    engine: &mut UciEngine,
    plies: &[crate::model::PlyRecord],
    cfg: &AnalysisConfig,
    cache: &mut HashMap<String, EngineSummary>,
) -> Result<Option<EngineSummary>> {
    let last_ply = match plies.last() {
        Some(ply) => ply,
        None => return Ok(None),
    };
    if last_ply.fen_after.is_empty() {
        return Ok(None);
    }

    let summary = analyze_position(
        engine,
        cache,
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
    Ok(Some(summary))
}

fn build_ply_analyses(
    plies: Vec<crate::model::PlyRecord>,
    summaries: &[EngineSummary],
    last_after_summary: Option<&EngineSummary>,
    clock_states: &[ClockState],
    cfg: &AnalysisConfig,
) -> Result<Vec<PlyAnalysis>> {
    let mut analyses: Vec<PlyAnalysis> = Vec::with_capacity(plies.len());

    for (idx, ply) in plies.into_iter().enumerate() {
        let summary = summaries
            .get(idx)
            .cloned()
            .ok_or_else(|| anyhow!("Missing engine summary for ply {}", idx + 1))?;

        let cp_eval_before = summary.best_cp_white.unwrap_or(0);
        let cp_eval_after = cp_eval_after_for_index(
            idx,
            summaries,
            last_after_summary,
            cp_eval_before,
        );
        let (tau_before, tau_after) = time_equity_for_ply(clock_states.get(idx), cfg, &ply);

        let cp_practical_before = cp_eval_before + tau_before;
        let cp_practical_after = cp_eval_after + tau_after;

        let p_eval_before = win_prob_from_cp(cfg.k_sigmoid, cp_eval_before);
        let p_eval_after = win_prob_from_cp(cfg.k_sigmoid, cp_eval_after);
        let p_practical_before = win_prob_from_cp(cfg.k_sigmoid, cp_practical_before);
        let p_practical_after = win_prob_from_cp(cfg.k_sigmoid, cp_practical_after);

        let dp_eval_mover =
            mover_prob(p_eval_after, ply.mover) - mover_prob(p_eval_before, ply.mover);
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
            summary.complexity_cp_mover,
            dp_practical_mover,
        );

        analyses.push(PlyAnalysis {
            ply,
            engine_before: summary,
            metrics,
            label,
        });
    }

    Ok(analyses)
}

fn cp_eval_after_for_index(
    idx: usize,
    summaries: &[EngineSummary],
    last_after_summary: Option<&EngineSummary>,
    fallback: i32,
) -> i32 {
    summaries
        .get(idx + 1)
        .and_then(|s| s.best_cp_white)
        .or_else(|| {
            if idx + 1 == summaries.len() {
                last_after_summary.and_then(|s| s.best_cp_white)
            } else {
                None
            }
        })
        .unwrap_or(fallback)
}

fn time_equity_for_ply(
    clocks: Option<&ClockState>,
    cfg: &AnalysisConfig,
    ply: &crate::model::PlyRecord,
) -> (i32, i32) {
    let tau_before = clocks
        .and_then(|c| match (c.before_white, c.before_black) {
            (Some(w), Some(b)) => Some(time_equity_white_cp(
                cfg.alpha,
                cfg.beta,
                cfg.time_pressure_pivot,
                cfg.time_pressure_scale,
                cfg.time_pressure_boost,
                w,
                b,
                ply.ply_index,
            )),
            _ => None,
        })
        .unwrap_or(0);
    let tau_after = clocks
        .and_then(|c| match (c.after_white, c.after_black) {
            (Some(w), Some(b)) => Some(time_equity_white_cp(
                cfg.alpha,
                cfg.beta,
                cfg.time_pressure_pivot,
                cfg.time_pressure_scale,
                cfg.time_pressure_boost,
                w,
                b,
                ply.ply_index,
            )),
            _ => None,
        })
        .unwrap_or(0);
    (tau_before, tau_after)
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

fn build_summary(analyses: &[PlyAnalysis], label_cfg: &LabelConfig) -> GameSummary {
    let mut label_counts: HashMap<String, u32> = HashMap::new();
    let mut think_times: Vec<f32> = Vec::new();
    let mut punish: Vec<i32> = Vec::new();
    let mut dp_practical: Vec<f32> = Vec::new();
    let mut complexities: Vec<i32> = Vec::new();

    let mut time_trouble_moves = 0u32;
    let mut panic_moves = 0u32;
    let mut blunders_in_time_trouble = 0u32;
    let mut known_clock_moves = 0u32;

    let mut phase_stats = [PhaseStats::default(), PhaseStats::default(), PhaseStats::default()];
    let mut total_think = 0.0f32;

    for ply in analyses {
        let key = format!("{:?}", ply.label.kind);
        *label_counts.entry(key).or_insert(0) += 1;

        if let Some(t) = ply.ply.think_time_secs {
            think_times.push(t);
            total_think += t;
        }
        if let Some(p) = ply.engine_before.punish_cp_mover {
            punish.push(p);
        }
        if let Some(c) = ply.engine_before.complexity_cp_mover {
            complexities.push(c);
        }
        dp_practical.push(ply.metrics.dp_practical_mover);

        if let Some(t_rem) = ply.ply.clock_before_secs {
            known_clock_moves += 1;
            if t_rem <= label_cfg.time_trouble_secs {
                time_trouble_moves += 1;
                let punish_val = ply.engine_before.punish_cp_mover.unwrap_or(0);
                if punish_val >= label_cfg.big_punish {
                    blunders_in_time_trouble += 1;
                }
            }
            if t_rem <= label_cfg.panic_secs {
                panic_moves += 1;
            }
        }

        let phase_idx = phase_index(ply.ply.ply_index);
        phase_stats[phase_idx].ply_count += 1;
        if let Some(t) = ply.ply.think_time_secs {
            phase_stats[phase_idx].think_sum += t;
            phase_stats[phase_idx].think_count += 1;
        }
        if let Some(c) = ply.engine_before.complexity_cp_mover {
            phase_stats[phase_idx].complexity_sum += c;
            phase_stats[phase_idx].complexity_count += 1;
        }
    }

    let avg_think_time = average_f32(&think_times);
    let avg_punish = average_i32(&punish);
    let avg_dp_practical = average_f32(&dp_practical);
    let avg_complexity = average_i32(&complexities);

    let phase_shares = phase_time_shares(&phase_stats, total_think);
    let phase_avgs = phase_avg_think_times(&phase_stats);
    let phase_complexity = phase_avg_complexity(&phase_stats);
    let phase_deltas = phase_share_deltas(&phase_shares);

    GameSummary {
        total_plies: analyses.len(),
        labels_count: label_counts,
        avg_think_time_secs: avg_think_time,
        avg_punish_cp_mover: avg_punish,
        avg_dp_practical_mover: avg_dp_practical,
        avg_complexity_cp_mover: avg_complexity,
        time_trouble_moves,
        panic_moves,
        blunders_in_time_trouble,
        time_trouble_rate: rate(time_trouble_moves, analyses.len()),
        panic_rate: rate(panic_moves, analyses.len()),
        time_trouble_rate_known: rate(time_trouble_moves, known_clock_moves as usize),
        panic_rate_known: rate(panic_moves, known_clock_moves as usize),
        phase_time_share: phase_shares,
        phase_time_share_delta_vs_15_70_15: phase_deltas,
        phase_avg_think_time_secs: phase_avgs,
        phase_avg_complexity_cp_mover: phase_complexity,
    }
}

#[derive(Clone, Copy, Default)]
struct PhaseStats {
    ply_count: u32,
    think_sum: f32,
    think_count: u32,
    complexity_sum: i32,
    complexity_count: u32,
}

fn phase_index(ply_index: u32) -> usize {
    if ply_index < PHASE_OPENING_END_PLY {
        0
    } else if ply_index < PHASE_MIDDLEGAME_END_PLY {
        1
    } else {
        2
    }
}

fn phase_time_shares(stats: &[PhaseStats; 3], total_think: f32) -> PhaseTimeShare {
    let opening = if total_think > 0.0 {
        stats[0].think_sum / total_think
    } else {
        0.0
    };
    let middlegame = if total_think > 0.0 {
        stats[1].think_sum / total_think
    } else {
        0.0
    };
    let endgame = if total_think > 0.0 {
        stats[2].think_sum / total_think
    } else {
        0.0
    };

    PhaseTimeShare {
        opening,
        middlegame,
        endgame,
    }
}

fn phase_avg_think_times(stats: &[PhaseStats; 3]) -> PhaseAverages {
    let opening = average_phase(stats[0]);
    let middlegame = average_phase(stats[1]);
    let endgame = average_phase(stats[2]);

    PhaseAverages {
        opening,
        middlegame,
        endgame,
    }
}

fn average_phase(stats: PhaseStats) -> Option<f32> {
    if stats.think_count == 0 {
        None
    } else {
        Some(stats.think_sum / stats.think_count as f32)
    }
}

fn phase_avg_complexity(stats: &[PhaseStats; 3]) -> PhaseAverages {
    let opening = average_phase_complexity(stats[0]);
    let middlegame = average_phase_complexity(stats[1]);
    let endgame = average_phase_complexity(stats[2]);

    PhaseAverages {
        opening,
        middlegame,
        endgame,
    }
}

fn average_phase_complexity(stats: PhaseStats) -> Option<f32> {
    if stats.complexity_count == 0 {
        None
    } else {
        Some(stats.complexity_sum as f32 / stats.complexity_count as f32)
    }
}

fn phase_share_deltas(shares: &PhaseTimeShare) -> PhaseTimeShareDelta {
    let opening = shares.opening - 0.15;
    let middlegame = shares.middlegame - 0.70;
    let endgame = shares.endgame - 0.15;

    PhaseTimeShareDelta {
        opening,
        middlegame,
        endgame,
    }
}

fn rate(count: u32, total: usize) -> Option<f32> {
    if total == 0 {
        None
    } else {
        Some(count as f32 / total as f32)
    }
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
