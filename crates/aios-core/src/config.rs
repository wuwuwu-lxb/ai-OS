//! Configuration management

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// AI-OS Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// AI Provider settings
    pub ai: AIConfig,
    /// Memory settings
    pub memory: MemoryConfig,
    /// CLI settings
    pub cli: CLIConfig,
}

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Context window size in tokens
    pub context_window_tokens: usize,
    /// Short-term memory retention in days
    pub short_term_retention_days: u32,
    /// Database path
    pub db_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CLIConfig {
    /// Prompt prefix
    pub prompt: String,
    /// Show timestamps
    pub show_timestamps: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ai: AIConfig {
                primary_provider: "claude".to_string(),
                anthropic_api_key: std::env::var("ANTHROPIC_API_KEY").ok(),
                openai_api_key: std::env::var("OPENAI_API_KEY").ok(),
                default_model: "claude-sonnet-4-20250514".to_string(),
                max_tokens: 4096,
                temperature: 0.7,
            },
            memory: MemoryConfig {
                context_window_tokens: 128_000,
                short_term_retention_days: 7,
                db_path: PathBuf::from(".aios/data"),
            },
            cli: CLIConfig {
                prompt: "❯ ".to_string(),
                show_timestamps: true,
            },
        }
    }
}

impl Config {
    /// Load configuration from file or use defaults
    pub fn load() -> Result<Self> {
        let config_path = PathBuf::from(".aios/config.toml");

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_path = PathBuf::from(".aios/config.toml");
        std::fs::create_dir_all(config_path.parent().unwrap())?;
        let content = toml::to_string_pretty(self)?;
        std::fs::write(config_path, content)?;
        Ok(())
    }
}
