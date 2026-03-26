//! AI Configuration for aios-ai crate

use serde::{Deserialize, Serialize};

/// AI Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    /// Primary provider: "claude" or "openai"
    pub primary_provider: String,
    /// API keys (can also be set via environment variables)
    pub anthropic_api_key: Option<String>,
    pub openai_api_key: Option<String>,
    /// Default model
    pub default_model: String,
    /// Max tokens per request
    pub max_tokens: u32,
    /// Temperature
    pub temperature: f32,
    /// Context window tokens
    pub context_window_tokens: usize,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            primary_provider: "claude".to_string(),
            anthropic_api_key: std::env::var("ANTHROPIC_API_KEY").ok(),
            openai_api_key: std::env::var("OPENAI_API_KEY").ok(),
            default_model: "claude-sonnet-4-20250514".to_string(),
            max_tokens: 4096,
            temperature: 0.7,
            context_window_tokens: 128_000,
        }
    }
}
