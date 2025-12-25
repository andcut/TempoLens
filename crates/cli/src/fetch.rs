use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy)]
pub enum FetchSource {
    Lichess,
    ChessCom,
}

pub async fn fetch_pgn(
    source: FetchSource,
    user: &str,
    games: usize,
    cache_dir: &Path,
    refresh: bool,
) -> Result<String> {
    let cache_path = cache_path(cache_dir, source, user, games);
    if !refresh {
        if let Ok(text) = fs::read_to_string(&cache_path) {
            if !text.trim().is_empty() {
                return Ok(text);
            }
        }
    }

    let text = match source {
        FetchSource::Lichess => fetch_lichess_pgn(user, games).await?,
        FetchSource::ChessCom => fetch_chesscom_pgn(user, games).await?,
    };

    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&cache_path, &text)?;

    Ok(text)
}

pub fn resolve_cache_dir(override_dir: Option<&PathBuf>) -> Result<PathBuf> {
    if let Some(dir) = override_dir {
        return Ok(dir.to_path_buf());
    }

    if let Ok(dir) = std::env::var("TIMELENS_CACHE_DIR") {
        return Ok(PathBuf::from(dir));
    }

    if let Ok(home) = std::env::var("HOME") {
        return Ok(PathBuf::from(home).join(".timelens/cache"));
    }

    if let Ok(home) = std::env::var("USERPROFILE") {
        return Ok(PathBuf::from(home).join(".timelens/cache"));
    }

    Ok(PathBuf::from(".timelens/cache"))
}

fn cache_path(base: &Path, source: FetchSource, user: &str, games: usize) -> PathBuf {
    let platform = match source {
        FetchSource::Lichess => "lichess",
        FetchSource::ChessCom => "chesscom",
    };
    base.join(platform)
        .join(user)
        .join(format!("last_{}.pgn", games))
}

async fn fetch_lichess_pgn(user: &str, games: usize) -> Result<String> {
    let client = client();
    let encoded = urlencoding::encode(user);
    let url = format!(
        "https://lichess.org/api/games/user/{}?max={}&clocks=true&evals=false&opening=false",
        encoded, games
    );

    let response = client
        .get(url)
        .header("Accept", "application/x-chess-pgn")
        .send()
        .await
        .context("Failed to call Lichess API")?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "Lichess API request failed with status {}",
            response.status()
        ));
    }

    let text = response.text().await?;
    if text.trim().is_empty() {
        return Err(anyhow!("Lichess returned empty PGN."));
    }

    Ok(text)
}

async fn fetch_chesscom_pgn(user: &str, games: usize) -> Result<String> {
    let client = client();
    let encoded = urlencoding::encode(user);
    let archives_url = format!(
        "https://api.chess.com/pub/player/{}/games/archives",
        encoded
    );

    let archives_resp = client
        .get(archives_url)
        .send()
        .await
        .context("Failed to call Chess.com archives API")?;
    if !archives_resp.status().is_success() {
        return Err(anyhow!(
            "Chess.com archives request failed with status {}",
            archives_resp.status()
        ));
    }

    let archives_json: Value = archives_resp.json().await?;
    let archives = archives_json
        .get("archives")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow!("Chess.com archives response missing 'archives'"))?;

    let mut pgns: Vec<String> = Vec::new();
    for archive in archives.iter().rev() {
        let archive_url = match archive.as_str() {
            Some(url) => url,
            None => continue,
        };

        let month_resp = client
            .get(archive_url)
            .send()
            .await
            .context("Failed to call Chess.com month archive")?;
        if !month_resp.status().is_success() {
            return Err(anyhow!(
                "Chess.com month archive failed with status {}",
                month_resp.status()
            ));
        }

        let month_json: Value = month_resp.json().await?;
        let games_array = month_json
            .get("games")
            .and_then(|v| v.as_array())
            .ok_or_else(|| anyhow!("Chess.com month response missing 'games'"))?;

        for game in games_array.iter().rev() {
            if let Some(pgn) = game.get("pgn").and_then(|v| v.as_str()) {
                pgns.push(pgn.to_string());
                if pgns.len() >= games {
                    break;
                }
            }
        }
        if pgns.len() >= games {
            break;
        }
    }

    if pgns.is_empty() {
        return Err(anyhow!("Chess.com returned no PGN data"));
    }

    pgns.reverse();
    Ok(pgns.join("\n\n\n"))
}

fn client() -> Client {
    Client::builder()
        .user_agent("TempoLens/0.1")
        .build()
        .expect("Failed to build HTTP client")
}
