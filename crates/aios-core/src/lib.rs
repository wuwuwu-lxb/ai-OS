//! AI-OS Core Library

mod config;
mod error;

pub use config::Config;
pub use error::AIOSError;

use anyhow::Result;
use tracing::info;

/// Main application entry point
pub async fn run() -> Result<()> {
    info!("Initializing AI-OS...");

    // Load configuration
    let config = Config::load()?;

    // Initialize memory store
    let memory = aios_memory::MemoryStore::new()?;
    info!("Memory store initialized");

    // Initialize AI middleware - convert config to aios-ai format
    let ai_config = aios_ai::AIConfig {
        primary_provider: config.ai.primary_provider.clone(),
        anthropic_api_key: config.ai.anthropic_api_key.clone(),
        openai_api_key: config.ai.openai_api_key.clone(),
        default_model: config.ai.default_model.clone(),
        max_tokens: config.ai.max_tokens,
        temperature: config.ai.temperature,
        context_window_tokens: config.memory.context_window_tokens,
    };
    let ai = aios_ai::AIMiddleware::new(&ai_config).await?;
    info!("AI middleware initialized");

    // Run CLI
    aios_cli::run(ai, memory).await?;

    Ok(())
}
