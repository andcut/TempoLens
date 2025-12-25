#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Deserialize;
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::Emitter;
use timelens_core::analysis::labeling::LabelConfig;
use timelens_core::pgn::parse_time_control_value;
use timelens_core::AnalysisConfig;

#[derive(Debug, Deserialize, Clone)]
struct AnalysisOptions {
    depth: Option<u16>,
    multipv: Option<u8>,
    movetime_ms: Option<u64>,
    threads: Option<u32>,
    hash_mb: Option<u32>,
    time_control: Option<String>,
    alpha: Option<f32>,
    beta: Option<f32>,
    time_pressure_pivot: Option<f32>,
    time_pressure_scale: Option<f32>,
    time_pressure_boost: Option<f32>,
    k_sigmoid: Option<f32>,
}

#[tauri::command]
async fn analyze_pgn_text(
    pgn: String,
    engine_path: String,
    options: Option<AnalysisOptions>,
) -> Result<String, String> {
    if engine_path.trim().is_empty() {
        return Err("Engine path is required.".to_string());
    }

    let options = options.unwrap_or(AnalysisOptions {
        depth: None,
        multipv: None,
        movetime_ms: None,
        threads: None,
        hash_mb: None,
        time_control: None,
        alpha: None,
        beta: None,
        time_pressure_pivot: None,
        time_pressure_scale: None,
        time_pressure_boost: None,
        k_sigmoid: None,
    });

    let fallback_time_control = if let Some(tc) = options.time_control.as_deref() {
        let parsed = parse_time_control_value(tc)
            .ok_or_else(|| format!("Invalid TimeControl '{}'. Use format like 180+2.", tc))?;
        Some(parsed)
    } else {
        None
    };

    let cfg = AnalysisConfig {
        engine_path,
        multipv: options.multipv.unwrap_or(4),
        depth: options.depth.unwrap_or(14),
        movetime_ms: options.movetime_ms,
        threads: options.threads,
        hash_mb: options.hash_mb,
        fallback_time_control,
        alpha: options.alpha.unwrap_or(2.0),
        beta: options.beta.unwrap_or(10.0),
        time_pressure_pivot: options.time_pressure_pivot.unwrap_or(30.0),
        time_pressure_scale: options.time_pressure_scale.unwrap_or(8.0),
        time_pressure_boost: options.time_pressure_boost.unwrap_or(3.0),
        k_sigmoid: options.k_sigmoid.unwrap_or(1.2),
        label_config: LabelConfig::default(),
    };

    let analysis = timelens_core::analysis::pipeline::analyze_pgn(&pgn, cfg)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&analysis).map_err(|e| e.to_string())
}

#[tauri::command]
async fn analyze_pgn_file(
    path: String,
    engine_path: String,
    options: Option<AnalysisOptions>,
) -> Result<String, String> {
    let pgn = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    analyze_pgn_text(pgn, engine_path, options).await
}

fn build_menu(app: &tauri::AppHandle) -> tauri::Result<Menu<tauri::Wry>> {
    let about = MenuItem::with_id(app, "about", "About TempoLens", true, None::<&str>)?;
    let preferences = MenuItem::with_id(app, "preferences", "Preferences", true, None::<&str>)?;
    let quit = PredefinedMenuItem::quit(app, None)?;

    let submenu = Submenu::with_items(
        app,
        "TempoLens",
        true,
        &[&about, &preferences, &quit],
    )?;

    Menu::with_items(app, &[&submenu])
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let menu = build_menu(app.handle())?;
            app.set_menu(menu)?;
            Ok(())
        })
        .on_menu_event(|app, event| match event.id().as_ref() {
            "about" => {
                let _ = app.emit("open-about", ());
            }
            "preferences" => {
                let _ = app.emit("open-settings", ());
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![analyze_pgn_text, analyze_pgn_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
