mod fetch;

use anyhow::{anyhow, Result};
use clap::Parser;
use std::path::PathBuf;
use timelens_core::analysis::labeling::LabelConfig;
use timelens_core::pgn::parse_time_control_value;
use timelens_core::AnalysisConfig;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    engine: String,
    #[arg(long)]
    pgn: Option<String>,
    #[arg(long)]
    lichess_user: Option<String>,
    #[arg(long)]
    chesscom_user: Option<String>,
    #[arg(long, default_value_t = 1)]
    games: usize,
    #[arg(long)]
    cache_dir: Option<PathBuf>,
    #[arg(long, default_value_t = false)]
    refresh_cache: bool,
    #[arg(long)]
    output: Option<String>,
    #[arg(long, default_value_t = 14)]
    depth: u16,
    #[arg(long, default_value_t = 4)]
    multipv: u8,
    #[arg(long)]
    movetime_ms: Option<u64>,
    #[arg(long)]
    threads: Option<u32>,
    #[arg(long)]
    hash_mb: Option<u32>,
    #[arg(long, default_value_t = 2.0)]
    alpha: f32,
    #[arg(long, default_value_t = 10.0)]
    beta: f32,
    #[arg(long, default_value_t = 30.0)]
    time_pressure_pivot: f32,
    #[arg(long, default_value_t = 8.0)]
    time_pressure_scale: f32,
    #[arg(long, default_value_t = 3.0)]
    time_pressure_boost: f32,
    #[arg(long, default_value_t = 1.2)]
    k_sigmoid: f32,
    #[arg(long)]
    time_control: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let source_count = args.pgn.is_some() as u8
        + args.lichess_user.is_some() as u8
        + args.chesscom_user.is_some() as u8;
    if source_count != 1 {
        return Err(anyhow!(
            "Provide exactly one source: --pgn, --lichess-user, or --chesscom-user."
        ));
    }
    if args.games == 0 {
        return Err(anyhow!("--games must be at least 1."));
    }

    let fallback_time_control = if let Some(tc) = args.time_control.as_deref() {
        match parse_time_control_value(tc) {
            Some(parsed) => Some(parsed),
            None => {
                return Err(anyhow!(
                    "Invalid --time-control value '{}'. Use format like 180+2.",
                    tc
                ));
            }
        }
    } else {
        None
    };

    let cfg = AnalysisConfig {
        engine_path: args.engine,
        multipv: args.multipv,
        depth: args.depth,
        movetime_ms: args.movetime_ms,
        threads: args.threads,
        hash_mb: args.hash_mb,
        fallback_time_control,
        alpha: args.alpha,
        beta: args.beta,
        time_pressure_pivot: args.time_pressure_pivot,
        time_pressure_scale: args.time_pressure_scale,
        time_pressure_boost: args.time_pressure_boost,
        k_sigmoid: args.k_sigmoid,
        label_config: LabelConfig::default(),
    };

    let pgn_text = load_pgn_text(&args).await?;
    let analyses = timelens_core::analysis::pipeline::analyze_pgns(&pgn_text, cfg).await?;
    let output = if analyses.len() == 1 {
        serde_json::to_string_pretty(&analyses[0])?
    } else {
        serde_json::to_string_pretty(&analyses)?
    };
    if let Some(path) = args.output {
        std::fs::write(path, output)?;
    } else {
        println!("{}", output);
    }

    Ok(())
}

async fn load_pgn_text(args: &Args) -> Result<String> {
    if let Some(path) = args.pgn.as_ref() {
        return Ok(std::fs::read_to_string(path)?);
    }

    let cache_dir = fetch::resolve_cache_dir(args.cache_dir.as_ref())?;

    if let Some(user) = args.lichess_user.as_ref() {
        return fetch::fetch_pgn(
            fetch::FetchSource::Lichess,
            user,
            args.games,
            &cache_dir,
            args.refresh_cache,
        )
        .await;
    }

    if let Some(user) = args.chesscom_user.as_ref() {
        return fetch::fetch_pgn(
            fetch::FetchSource::ChessCom,
            user,
            args.games,
            &cache_dir,
            args.refresh_cache,
        )
        .await;
    }

    Err(anyhow!("No PGN source provided."))
}
