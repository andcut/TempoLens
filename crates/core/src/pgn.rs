use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use pgn_reader::{BufferedReader, RawComment, RawHeader, SanPlus, Skip, Visitor};
use regex::Regex;
use std::collections::HashMap;

use crate::model::{SourcePlatform, TimeControl};

#[derive(Debug, Clone)]
pub struct ParsedGame {
    pub headers: HashMap<String, String>,
    pub plies: Vec<RawPly>,
}

#[derive(Debug, Clone)]
pub struct RawPly {
    pub san: String,
    pub clock_after_secs: Option<f32>,
    pub comment: Option<String>,
}

pub fn parse_single_game(pgn: &str) -> Result<ParsedGame> {
    let mut games = parse_games(pgn)?;
    if games.is_empty() {
        return Err(anyhow!("No PGN game found in input"));
    }
    Ok(games.remove(0))
}

pub fn parse_games(pgn: &str) -> Result<Vec<ParsedGame>> {
    let mut reader = BufferedReader::new_cursor(pgn.as_bytes());
    let mut games = Vec::new();

    loop {
        let mut visitor = GameVisitor::default();
        match reader.read_game(&mut visitor)? {
            Some(game) => games.push(game),
            None => break,
        }
    }

    Ok(games)
}

#[derive(Default)]
struct GameVisitor {
    headers: HashMap<String, String>,
    plies: Vec<RawPly>,
}

impl Visitor for GameVisitor {
    type Result = ParsedGame;

    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        let k = String::from_utf8_lossy(key).to_string();
        let v = String::from_utf8_lossy(value.as_bytes()).to_string();
        self.headers.insert(k, v);
    }

    fn san(&mut self, san: SanPlus) {
        self.plies.push(RawPly {
            san: san.to_string(),
            clock_after_secs: None,
            comment: None,
        });
    }

    fn comment(&mut self, comment: RawComment<'_>) {
        if let Some(last) = self.plies.last_mut() {
            let s = String::from_utf8_lossy(comment.as_bytes()).to_string();
            last.clock_after_secs = parse_clk_comment_secs(&s).or(last.clock_after_secs);
            last.comment = Some(s);
        }
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true)
    }

    fn end_game(&mut self) -> Self::Result {
        ParsedGame {
            headers: std::mem::take(&mut self.headers),
            plies: std::mem::take(&mut self.plies),
        }
    }
}

pub fn detect_platform(headers: &HashMap<String, String>) -> SourcePlatform {
    let site = headers.get("Site").map(|s| s.to_lowercase());
    let event = headers.get("Event").map(|s| s.to_lowercase());

    let hint = site.as_ref().or(event.as_ref());
    if let Some(h) = hint {
        if h.contains("lichess") {
            return SourcePlatform::Lichess;
        }
        if h.contains("chess.com") || h.contains("chesscom") {
            return SourcePlatform::ChessCom;
        }
    }

    SourcePlatform::Unknown
}

pub fn parse_clk_comment_secs(comment: &str) -> Option<f32> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"\[%clk\s*([0-9]+):([0-9]{1,2})(?::([0-9]{1,2}))?\]").unwrap()
    });

    let caps = RE.captures(comment)?;
    let a: u32 = caps.get(1)?.as_str().parse().ok()?;
    let b: u32 = caps.get(2)?.as_str().parse().ok()?;
    let c_opt = caps.get(3).and_then(|m| m.as_str().parse::<u32>().ok());
    if b > 59 {
        return None;
    }
    if let Some(c) = c_opt {
        if c > 59 {
            return None;
        }
    }

    let secs = if let Some(c) = c_opt {
        (a * 3600 + b * 60 + c) as f32
    } else {
        (a * 60 + b) as f32
    };

    Some(secs)
}

pub fn parse_time_control_header(headers: &HashMap<String, String>) -> Option<TimeControl> {
    let tc = headers.get("TimeControl")?;
    parse_time_control_value(tc)
}

pub fn parse_time_control_value(tc: &str) -> Option<TimeControl> {
    if tc == "-" {
        return None;
    }
    let mut parts = tc.split('+');
    let base = parts.next()?.parse::<u32>().ok()?;
    let inc = parts.next().and_then(|v| v.parse::<u32>().ok()).unwrap_or(0);
    Some(TimeControl {
        base_secs: base,
        increment_secs: inc,
    })
}
