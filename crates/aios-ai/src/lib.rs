//! AI Middleware - AI Provider abstraction and orchestration

mod gateway;
mod provider;
mod cache;
mod context;
mod config;

pub use gateway::{AIMiddleware, AIRequest, AIResponse, Message, Model};
pub use provider::{AIProvider, ProviderError};
pub use context::ContextManager;
pub use cache::AICache;
pub use config::AIConfig;

use anyhow::Result;

/// Create a new AI middleware instance
pub async fn new_ai_middleware(config: &AIConfig) -> Result<AIMiddleware> {
    AIMiddleware::new(config).await
}
