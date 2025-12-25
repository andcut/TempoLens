export type Color = "White" | "Black";

export type SourcePlatform = "Lichess" | "ChessCom" | "Unknown";

export interface TimeControl {
  base_secs: number;
  increment_secs: number;
}

export interface PlyRecord {
  ply_index: number;
  san: string;
  uci: string;
  mover: Color;
  fen_before: string;
  fen_after: string;
  clock_after_secs?: number | null;
  clock_before_secs?: number | null;
  think_time_secs?: number | null;
}

export interface EngineLine {
  multipv: number;
  uci: string;
  cp_white: number;
  mate?: number | null;
}

export interface EngineSummary {
  depth: number;
  nodes: number;
  nps: number;
  lines: EngineLine[];
  played_cp_white?: number | null;
  best_cp_white?: number | null;
  punish_cp_mover?: number | null;
  spread_k_cp_mover?: number | null;
  gap_12_cp_mover?: number | null;
  complexity_cp_mover?: number | null;
}

export interface Label {
  kind: string;
  severity: number;
  title: string;
  explanation: string;
  tips: string[];
}

export interface MoveMetrics {
  tau_white_cp: number;
  cp_eval_before: number;
  cp_eval_after: number;
  cp_practical_before: number;
  cp_practical_after: number;
  p_eval_before: number;
  p_eval_after: number;
  p_practical_before: number;
  p_practical_after: number;
  dp_eval_mover: number;
  dp_practical_mover: number;
}

export interface PlyAnalysis {
  ply: PlyRecord;
  engine_before: EngineSummary;
  metrics: MoveMetrics;
  label: Label;
}

export interface GameMeta {
  event?: string;
  site?: string;
  date?: string;
  round?: string;
  white?: string;
  black?: string;
  result?: string;
  time_control?: TimeControl | null;
  platform: SourcePlatform;
  headers?: Record<string, string>;
}

export interface GameSummary {
  total_plies: number;
  labels_count: Record<string, number>;
  avg_think_time_secs?: number | null;
  avg_punish_cp_mover?: number | null;
  avg_dp_practical_mover?: number | null;
  avg_complexity_cp_mover?: number | null;
  time_trouble_moves: number;
  panic_moves: number;
  blunders_in_time_trouble: number;
  time_trouble_rate?: number | null;
  panic_rate?: number | null;
  time_trouble_rate_known?: number | null;
  panic_rate_known?: number | null;
  phase_time_share: {
    opening: number;
    middlegame: number;
    endgame: number;
  };
  phase_time_share_delta_vs_15_70_15: {
    opening: number;
    middlegame: number;
    endgame: number;
  };
  phase_avg_think_time_secs: {
    opening?: number | null;
    middlegame?: number | null;
    endgame?: number | null;
  };
  phase_avg_complexity_cp_mover: {
    opening?: number | null;
    middlegame?: number | null;
    endgame?: number | null;
  };
}

export interface GameAnalysis {
  meta: GameMeta;
  plies: PlyAnalysis[];
  summary: GameSummary;
}
