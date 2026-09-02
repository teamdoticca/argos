use thiserror::Error;

pub type Result<T> = std::result::Result<T, ArgosError>;

#[derive(Debug, Error)]
pub enum ArgosError {
    #[error("root path does not exist: {0}")]
    RootMissing(String),
    #[error("root path is not a directory: {0}")]
    RootNotDirectory(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse error: {0}")]
    Parse(String),
    #[error("invalid snapshot delta: {0}")]
    InvalidDelta(String),
    #[error("watch not started")]
    WatchNotStarted,
    #[error("unsupported on this platform: {0}")]
    UnsupportedPlatform(String),
    #[error("{0}")]
    Message(String),
}
