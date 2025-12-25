use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::{Child, Command},
    time::{timeout, Duration},
};

use crate::engine::{parse::UciInfoAccumulator, EngineError};
use crate::model::EngineSummary;

pub struct UciEngine {
    _child: Child,
    stdin: tokio::process::ChildStdin,
    stdout: BufReader<tokio::process::ChildStdout>,
}

impl UciEngine {
    pub async fn start(path: &str) -> Result<Self, EngineError> {
        let mut child = Command::new(path)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| EngineError::Protocol("missing stdin".into()))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| EngineError::Protocol("missing stdout".into()))?;

        let mut engine = Self {
            _child: child,
            stdin,
            stdout: BufReader::new(stdout),
        };

        engine.send("uci").await?;
        engine.wait_for("uciok", Duration::from_secs(2)).await?;
        engine.send("isready").await?;
        engine.wait_for("readyok", Duration::from_secs(2)).await?;

        Ok(engine)
    }

    pub async fn set_option(&mut self, name: &str, value: &str) -> Result<(), EngineError> {
        self.send(&format!("setoption name {} value {}", name, value))
            .await?;
        Ok(())
    }

    pub async fn new_game(&mut self) -> Result<(), EngineError> {
        self.send("ucinewgame").await?;
        self.send("isready").await?;
        self.wait_for("readyok", Duration::from_secs(2)).await?;
        Ok(())
    }

    pub async fn position_fen(&mut self, fen: &str) -> Result<(), EngineError> {
        self.send(&format!("position fen {}", fen)).await?;
        Ok(())
    }

    pub async fn go_multipv(
        &mut self,
        depth: u16,
        movetime_ms: Option<u64>,
        multipv: u8,
        searchmoves: Option<&str>,
    ) -> Result<EngineSummary, EngineError> {
        let mut cmd = if let Some(ms) = movetime_ms {
            format!("go movetime {}", ms)
        } else {
            format!("go depth {}", depth)
        };

        if let Some(sm) = searchmoves {
            cmd.push_str(" searchmoves ");
            cmd.push_str(sm);
        }

        self.send(&cmd).await?;

        let mut acc = UciInfoAccumulator::new(multipv);
        loop {
            let line = self.read_line(Duration::from_secs(10)).await?;
            if line.starts_with("info ") {
                acc.ingest_line(&line);
            } else if line.starts_with("bestmove ") {
                break;
            }
        }

        Ok(acc.into_summary())
    }

    pub async fn shutdown(mut self) -> Result<(), EngineError> {
        let _ = self.send("quit").await;
        Ok(())
    }

    async fn send(&mut self, s: &str) -> Result<(), EngineError> {
        self.stdin.write_all(s.as_bytes()).await?;
        self.stdin.write_all(b"\n").await?;
        self.stdin.flush().await?;
        Ok(())
    }

    async fn wait_for(&mut self, token: &str, dur: Duration) -> Result<(), EngineError> {
        let fut = async {
            loop {
                let mut buf = String::new();
                let n = self.stdout.read_line(&mut buf).await?;
                if n == 0 {
                    return Err(EngineError::Protocol("engine exited".into()));
                }
                if buf.trim() == token {
                    return Ok(());
                }
            }
        };
        timeout(dur, fut)
            .await
            .map_err(|_| EngineError::Timeout)??;
        Ok(())
    }

    async fn read_line(&mut self, dur: Duration) -> Result<String, EngineError> {
        let fut = async {
            let mut buf = String::new();
            let n = self.stdout.read_line(&mut buf).await?;
            if n == 0 {
                return Err(EngineError::Protocol("engine exited".into()));
            }
            Ok(buf.trim().to_string())
        };
        Ok(timeout(dur, fut)
            .await
            .map_err(|_| EngineError::Timeout)??)
    }
}

impl Drop for UciEngine {
    fn drop(&mut self) {
        let _ = self._child.start_kill();
    }
}
