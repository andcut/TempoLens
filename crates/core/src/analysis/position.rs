use anyhow::{anyhow, Result};
use shakmaty::{
    fen::Fen,
    san::San,
    CastlingMode, Chess, EnPassantMode, Position,
};

use crate::model::{Color, PlyRecord};
use crate::pgn::ParsedGame;

pub fn build_ply_records_with_fens(game: &ParsedGame) -> Result<Vec<PlyRecord>> {
    let mut pos = Chess::default();
    let mut out: Vec<PlyRecord> = Vec::with_capacity(game.plies.len());

    for (idx, raw) in game.plies.iter().enumerate() {
        let mover = match pos.turn() {
            shakmaty::Color::White => Color::White,
            shakmaty::Color::Black => Color::Black,
        };

        let fen_before = fen_string(&pos);

        let san: San = raw
            .san
            .parse()
            .map_err(|_| anyhow!("Failed to parse SAN '{}' at ply {}", raw.san, idx + 1))?;

        let mv = san.to_move(&pos).map_err(|e| {
            anyhow!(
                "Illegal/ambiguous SAN '{}' at ply {}: {}",
                raw.san,
                idx + 1,
                e
            )
        })?;

        let uci = mv.to_uci(CastlingMode::Standard).to_string();

        let pos_after = pos
            .play(&mv)
            .map_err(|e| anyhow!("Failed to apply move at ply {}: {}", idx + 1, e))?;

        let fen_after = fen_string(&pos_after);

        out.push(PlyRecord {
            ply_index: (idx + 1) as u32,
            san: raw.san.clone(),
            uci,
            mover,
            fen_before,
            fen_after,
            clock_after_secs: raw.clock_after_secs,
            clock_before_secs: None,
            think_time_secs: None,
        });

        pos = pos_after;
    }

    Ok(out)
}

fn fen_string<P: Position + Clone>(pos: &P) -> String {
    Fen::from_position(pos.clone(), EnPassantMode::Legal).to_string()
}
