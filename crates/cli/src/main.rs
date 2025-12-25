use clap::Parser;
use timelens_core::analysis::labeling::LabelConfig;
use timelens_core::pgn::parse_time_control_value;
use timelens_core::AnalysisConfig;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    engine: String,
    #[arg(long)]
    pgn: String,
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
    #[arg(long, default_value_t = 1.2)]
    k_sigmoid: f32,
    #[arg(long)]
    time_control: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let fallback_time_control = if let Some(tc) = args.time_control.as_deref() {
        match parse_time_control_value(tc) {
            Some(parsed) => Some(parsed),
            None => {
                return Err(anyhow::anyhow!(
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
        k_sigmoid: args.k_sigmoid,
        label_config: LabelConfig::default(),
    };

    let pgn_text = std::fs::read_to_string(&args.pgn)?;
    let analysis = timelens_core::analysis::pipeline::analyze_pgn(&pgn_text, cfg).await?;

    let output = serde_json::to_string_pretty(&analysis)?;
    if let Some(path) = args.output {
        std::fs::write(path, output)?;
    } else {
        println!("{}", output);
    }

    Ok(())
}
