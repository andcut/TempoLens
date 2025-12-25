use crate::model::{EngineLine, EngineSummary};

pub struct UciInfoAccumulator {
    target_multipv: u8,
    lines: Vec<Option<EngineLine>>,
    line_depths: Vec<u16>,
    depth: u16,
    nodes: u64,
    nps: u64,
}

impl UciInfoAccumulator {
    pub fn new(target_multipv: u8) -> Self {
        Self {
            target_multipv,
            lines: vec![None; target_multipv as usize],
            line_depths: vec![0; target_multipv as usize],
            depth: 0,
            nodes: 0,
            nps: 0,
        }
    }

    pub fn ingest_line(&mut self, s: &str) {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        if tokens.is_empty() || tokens[0] != "info" {
            return;
        }

        let mut i = 1;
        let mut depth: Option<u16> = None;
        let mut multipv: u8 = 1;
        let mut score_kind: Option<&str> = None;
        let mut score_val: Option<i32> = None;
        let mut first_pv: Option<String> = None;

        while i < tokens.len() {
            match tokens[i] {
                "depth" => {
                    if i + 1 < tokens.len() {
                        depth = tokens[i + 1].parse::<u16>().ok();
                    }
                    i += 2;
                }
                "multipv" => {
                    if i + 1 < tokens.len() {
                        multipv = tokens[i + 1].parse::<u8>().unwrap_or(1);
                    }
                    i += 2;
                }
                "score" => {
                    if i + 2 < tokens.len() {
                        score_kind = Some(tokens[i + 1]);
                        score_val = tokens[i + 2].parse::<i32>().ok();
                    }
                    i += 3;
                }
                "nodes" => {
                    if i + 1 < tokens.len() {
                        self.nodes = tokens[i + 1].parse::<u64>().unwrap_or(self.nodes);
                    }
                    i += 2;
                }
                "nps" => {
                    if i + 1 < tokens.len() {
                        self.nps = tokens[i + 1].parse::<u64>().unwrap_or(self.nps);
                    }
                    i += 2;
                }
                "pv" => {
                    if i + 1 < tokens.len() {
                        first_pv = Some(tokens[i + 1].to_string());
                    }
                    break;
                }
                _ => i += 1,
            }
        }

        if let Some(d) = depth {
            if d > self.depth {
                self.depth = d;
            }
        }

        if let (Some(kind), Some(val), Some(pv)) = (score_kind, score_val, first_pv) {
            let (cp_white, mate) = if kind == "cp" {
                (val, None)
            } else {
                let sign = if val >= 0 { 1 } else { -1 };
                (sign * 100_000, Some(val))
            };

            if multipv >= 1 && multipv <= self.target_multipv {
                let idx = (multipv - 1) as usize;
                let depth_val = depth.unwrap_or(0);
                if depth_val >= self.line_depths[idx] {
                    self.line_depths[idx] = depth_val;
                    self.lines[idx] = Some(EngineLine {
                        multipv,
                        uci: pv,
                        cp_white,
                        mate,
                    });
                }
            }
        }
    }

    pub fn into_summary(self) -> EngineSummary {
        let mut lines: Vec<EngineLine> = self.lines.into_iter().flatten().collect();
        lines.sort_by_key(|l| l.multipv);

        EngineSummary {
            depth: self.depth,
            nodes: self.nodes,
            nps: self.nps,
            lines,
            played_cp_white: None,
            best_cp_white: None,
            punish_cp_mover: None,
            spread_k_cp_mover: None,
        }
    }
}
