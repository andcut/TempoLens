#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Deserialize;
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::Emitter;
use timelens_core::analysis::labeling::LabelConfig;
use timelens_core::pgn::parse_time_control_value;
use timelens_core::AnalysisConfig;

#[derive(Debug, Deserialize, Clone, Default)]
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

impl AnalysisOptions {
    fn to_config(&self, engine_path: String) -> Result<AnalysisConfig, String> {
        let base = AnalysisConfig::default();
        
        let fallback_time_control = if let Some(tc) = &self.time_control {
            let parsed = parse_time_control_value(tc)
                .ok_or_else(|| format!("Invalid TimeControl '{}'. Use format like 180+2.", tc))?;
            Some(parsed)
        } else {
            None
        };

        Ok(AnalysisConfig {
            engine_path,
            multipv: self.multipv.unwrap_or(base.multipv),
            depth: self.depth.unwrap_or(base.depth),
            movetime_ms: self.movetime_ms.or(base.movetime_ms),
            threads: self.threads.or(base.threads),
            hash_mb: self.hash_mb.or(base.hash_mb),
            fallback_time_control,
            alpha: self.alpha.unwrap_or(base.alpha),
            beta: self.beta.unwrap_or(base.beta),
            time_pressure_pivot: self.time_pressure_pivot.unwrap_or(base.time_pressure_pivot),
            time_pressure_scale: self.time_pressure_scale.unwrap_or(base.time_pressure_scale),
            time_pressure_boost: self.time_pressure_boost.unwrap_or(base.time_pressure_boost),
            k_sigmoid: self.k_sigmoid.unwrap_or(base.k_sigmoid),
            label_config: base.label_config,
        })
    }
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

    let options = options.unwrap_or_default();
    let cfg = options.to_config(engine_path)?;

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
