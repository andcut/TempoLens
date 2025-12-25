#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use timelens_core::analysis::labeling::LabelConfig;
use timelens_core::AnalysisConfig;

#[tauri::command]
async fn analyze_pgn_text(pgn: String, engine_path: String, depth: Option<u16>) -> Result<String, String> {
    let depth = depth.unwrap_or(14);
    println!("[Tauri] Starting analysis with engine: {}, depth: {}", engine_path, depth);
    
    let cfg = AnalysisConfig {
        engine_path,
        multipv: 4,
        depth,
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
    };

    let analysis = timelens_core::analysis::pipeline::analyze_pgn(&pgn, cfg)
        .await
        .map_err(|e| e.to_string())?;

    println!("[Tauri] Analysis complete, {} plies processed at depth {}", analysis.plies.len(), depth);
    serde_json::to_string(&analysis).map_err(|e| e.to_string())
}

#[tauri::command]
async fn analyze_pgn_file(path: String, engine_path: String, depth: Option<u16>) -> Result<String, String> {
    let pgn = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    analyze_pgn_text(pgn, engine_path, depth).await
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![analyze_pgn_text, analyze_pgn_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
