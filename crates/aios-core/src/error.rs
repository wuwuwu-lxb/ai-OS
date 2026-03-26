//! Error types for AI-OS

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AIOSError {
    #[error("AI provider error: {0}")]
    AIError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Process error: {0}")]
    ProcessError(String),

    #[error("Memory error: {0}")]
    MemoryError(String),
}
