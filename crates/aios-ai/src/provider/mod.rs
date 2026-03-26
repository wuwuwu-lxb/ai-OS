//! AI Provider implementations

mod claude;
mod openai;

pub use claude::AnthropicProvider;
pub use openai::OpenAIProvider;
pub use crate::gateway::{AIRequest, AIResponse, Model};

use anyhow::Result;
use async_trait::async_trait;

/// AI Provider trait - pluggable backends
#[async_trait]
pub trait AIProvider: Send + Sync {
    /// Send a completion request
    async fn complete(&self, req: &AIRequest) -> Result<AIResponse>;

    /// Get provider name
    fn name(&self) -> &str;
}

/// Provider-specific errors
#[derive(thiserror::Error, Debug)]
pub enum ProviderError {
    #[error("API error: {0}")]
    ApiError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Rate limit error: {0}")]
    RateLimitError(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Parse error: {0}")]
    ParseError(String),
}

impl From<reqwest::Error> for ProviderError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            ProviderError::NetworkError("Request timeout".to_string())
        } else if e.is_connect() {
            ProviderError::NetworkError("Connection failed".to_string())
        } else {
            ProviderError::ApiError(e.to_string())
        }
    }
}
