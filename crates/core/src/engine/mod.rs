pub mod parse;
pub mod uci;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("protocol error: {0}")]
    Protocol(String),
    #[error("timeout")]
    Timeout,
}
