export type Color = "White" | "Black";

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
  metrics: MoveMetrics;
  label: Label;
}

export interface GameAnalysis {
  meta: {
    white?: string;
    black?: string;
    result?: string;
    platform: string;
  };
  plies: PlyAnalysis[];
}
