//! AI Gateway - Main orchestration layer

use crate::provider::{AIProvider, AnthropicProvider, OpenAIProvider};
use crate::cache::AICache;
use crate::context::ContextManager;
use crate::config::AIConfig;
use anyhow::{Result, Context};
use std::sync::Arc;
use tracing::{info, warn, debug};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// AI Middleware - Cloud-first with local fallback
pub struct AIMiddleware {
    primary: Arc<dyn AIProvider>,
    fallback: Option<Arc<dyn AIProvider>>,
    cache: Arc<AICache>,
    context: ContextManager,
    config: AIConfig,
}

impl AIMiddleware {
    /// Create a new AI middleware instance
    pub async fn new(config: &AIConfig) -> Result<Self> {
        // Initialize primary provider
        let primary: Arc<dyn AIProvider> = match config.primary_provider.as_str() {
            "claude" => {
                let api_key = config
                    .anthropic_api_key
                    .clone()
                    .context("ANTHROPIC_API_KEY not set")?;
                Arc::new(AnthropicProvider::new(&api_key, &config.default_model)?)
            }
            "openai" => {
                let api_key = config
                    .openai_api_key
                    .clone()
                    .context("OPENAI_API_KEY not set")?;
                Arc::new(OpenAIProvider::new(&api_key, &config.default_model)?)
            }
            _ => anyhow::bail!("Unknown provider: {}", config.primary_provider),
        };

        info!("Primary AI provider initialized: {}", config.primary_provider);

        Ok(Self {
            primary,
            fallback: None,
            cache: Arc::new(AICache::new()),
            context: ContextManager::new(config.context_window_tokens),
            config: config.clone(),
        })
    }

    /// Send a conversation message and get AI response
    pub async fn complete(&self, prompt: &str) -> Result<AIResponse> {
        let request_id = Uuid::new_v4();

        debug!("Processing request {}: {}", request_id, prompt);

        // Check cache
        if let Some(cached) = self.cache.get(prompt) {
            info!("Cache hit for request {}", request_id);
            return Ok(cached);
        }

        // Build request
        let messages = vec![
            Message {
                role: Role::User,
                content: prompt.to_string(),
                timestamp: Utc::now(),
            }
        ];

        let request = AIRequest {
            messages,
            model: Model::Claude {
                version: self.config.default_model.clone(),
            },
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
        };

        // Try primary provider
        match self.primary.complete(&request).await {
            Ok(response) => {
                debug!("Primary provider succeeded");
                self.cache.insert(prompt.to_string(), response.clone());
                Ok(response)
            }
            Err(e) => {
                warn!("Primary provider failed: {}, trying fallback", e);

                // Try fallback if available
                if let Some(fallback) = &self.fallback {
                    fallback.complete(&request).await
                } else {
                    Err(e)
                }
            }
        }
    }

    /// Add a message to context history
    pub fn add_to_context(&mut self, role: Role, content: String) {
        self.context.add_message(role, content);
    }

    /// Get current context
    pub fn get_context(&self) -> Vec<Message> {
        self.context.get_messages()
    }
}

/// AI Request structure
#[derive(Debug, Clone)]
pub struct AIRequest {
    pub messages: Vec<Message>,
    pub model: Model,
    pub max_tokens: u32,
    pub temperature: f32,
}

/// AI Response structure
#[derive(Debug, Clone)]
pub struct AIResponse {
    pub content: String,
    pub model: String,
    pub usage: Usage,
    pub reasoning: Option<String>,
}

/// Usage statistics
#[derive(Debug, Clone)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

/// Message in a conversation
#[derive(Debug, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

/// Message role
#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    User,
    Assistant,
    System,
}

/// Model selection
#[derive(Debug, Clone)]
pub enum Model {
    Claude { version: String },
    OpenAI { version: String },
}
