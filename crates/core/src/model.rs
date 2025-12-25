use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeControl {
    pub base_secs: u32,
    pub increment_secs: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SourcePlatform {
    Lichess,
    ChessCom,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMeta {
    pub event: Option<String>,
    pub site: Option<String>,
    pub date: Option<String>,
    pub round: Option<String>,
    pub white: Option<String>,
    pub black: Option<String>,
    pub result: Option<String>,
    pub time_control: Option<TimeControl>,
    pub platform: SourcePlatform,
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlyRecord {
    pub ply_index: u32,
    pub san: String,
    pub uci: String,
    pub mover: Color,
    pub fen_before: String,
    pub fen_after: String,

    pub clock_after_secs: Option<f32>,
    pub clock_before_secs: Option<f32>,
    pub think_time_secs: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineLine {
    pub multipv: u8,
    pub uci: String,
    pub cp_white: i32,
    pub mate: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineSummary {
    pub depth: u16,
    pub nodes: u64,
    pub nps: u64,
    pub lines: Vec<EngineLine>,

    pub played_cp_white: Option<i32>,
    pub best_cp_white: Option<i32>,
    pub punish_cp_mover: Option<i32>,
    pub spread_k_cp_mover: Option<i32>,
    pub gap_12_cp_mover: Option<i32>,
    pub complexity_cp_mover: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveMetrics {
    pub tau_white_cp: i32,
    pub cp_eval_before: i32,
    pub cp_eval_after: i32,
    pub cp_practical_before: i32,
    pub cp_practical_after: i32,

    pub p_eval_before: f32,
    pub p_eval_after: f32,
    pub p_practical_before: f32,
    pub p_practical_after: f32,

    pub dp_eval_mover: f32,
    pub dp_practical_mover: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LabelKind {
    OverthinkSimple,
    UnderthinkCritical,
    WastedThink,
    GoodInvestment,
    SnapBlunder,
    PanicBlunder,
    TimeBlunder,
    TimeTrouble,
    Neutral,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub kind: LabelKind,
    pub severity: f32,
    pub title: String,
    pub explanation: String,
    pub tips: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlyAnalysis {
    pub ply: PlyRecord,
    pub engine_before: EngineSummary,
    pub metrics: MoveMetrics,
    pub label: Label,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameAnalysis {
    pub meta: GameMeta,
    pub plies: Vec<PlyAnalysis>,
    pub summary: GameSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseTimeShare {
    pub opening: f32,
    pub middlegame: f32,
    pub endgame: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseTimeShareDelta {
    pub opening: f32,
    pub middlegame: f32,
    pub endgame: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseAverages {
    pub opening: Option<f32>,
    pub middlegame: Option<f32>,
    pub endgame: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSummary {
    pub total_plies: usize,
    pub labels_count: HashMap<String, u32>,
    pub avg_think_time_secs: Option<f32>,
    pub avg_punish_cp_mover: Option<f32>,
    pub avg_dp_practical_mover: Option<f32>,
    pub avg_complexity_cp_mover: Option<f32>,
    pub time_trouble_moves: u32,
    pub panic_moves: u32,
    pub blunders_in_time_trouble: u32,
    pub time_trouble_rate: Option<f32>,
    pub panic_rate: Option<f32>,
    pub time_trouble_rate_known: Option<f32>,
    pub panic_rate_known: Option<f32>,
    pub phase_time_share: PhaseTimeShare,
    pub phase_time_share_delta_vs_15_70_15: PhaseTimeShareDelta,
    pub phase_avg_think_time_secs: PhaseAverages,
    pub phase_avg_complexity_cp_mover: PhaseAverages,
}
