//! Anthropic Claude API provider

use super::{AIProvider, ProviderError};
use crate::gateway::{AIRequest, AIResponse, Message, Model, Role, Usage};
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

const ANTHROPIC_API_URL: &str = "https://api.anthropic.com/v1/messages";

pub struct AnthropicProvider {
    client: Client,
    api_key: String,
    model: String,
}

impl AnthropicProvider {
    pub fn new(api_key: &str, model: &str) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()?;

        Ok(Self {
            client,
            api_key: api_key.to_string(),
            model: model.to_string(),
        })
    }
}

#[derive(Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<ClaudeMessage>,
    temperature: f32,
}

#[derive(Serialize)]
struct ClaudeMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ClaudeResponse {
    content: Vec<ClaudeContent>,
    id: String,
    model: String,
    usage: ClaudeUsage,
}

#[derive(Deserialize)]
struct ClaudeContent {
    #[serde(rename = "type")]
    content_type: String,
    text: Option<String>,
}

#[derive(Deserialize)]
struct ClaudeUsage {
    input_tokens: u32,
    output_tokens: u32,
}

#[async_trait]
impl AIProvider for AnthropicProvider {
    async fn complete(&self, req: &AIRequest) -> Result<AIResponse> {
        let Model::Claude { version } = &req.model else {
            anyhow::bail!("Invalid model for Claude provider");
        };

        // Convert messages
        let messages: Vec<ClaudeMessage> = req
            .messages
            .iter()
            .map(|m| {
                let role = match m.role {
                    Role::User => "user",
                    Role::Assistant => "assistant",
                    Role::System => "system",
                };
                ClaudeMessage {
                    role: role.to_string(),
                    content: m.content.clone(),
                }
            })
            .collect();

        let claude_req = ClaudeRequest {
            model: version.clone(),
            max_tokens: req.max_tokens,
            messages,
            temperature: req.temperature,
        };

        debug!("Sending request to Claude API");

        let response = self
            .client
            .post(ANTHROPIC_API_URL)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&claude_req)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            info!("Claude API error: {} - {}", status, error_text);

            if status.as_u16() == 401 {
                return Err(ProviderError::AuthError("Invalid API key".to_string()).into());
            } else if status.as_u16() == 429 {
                return Err(ProviderError::RateLimitError("Rate limit exceeded".to_string()).into());
            }
            return Err(ProviderError::ApiError(format!("{}: {}", status, error_text)).into());
        }

        let claude_resp: ClaudeResponse = response.json().await
            .context("Failed to parse Claude response")?;

        let content = claude_resp
            .content
            .iter()
            .filter_map(|c| c.text.clone())
            .collect::<Vec<_>>()
            .join("\n");

        Ok(AIResponse {
            content,
            model: format!("claude-{}", claude_resp.model),
            usage: Usage {
                input_tokens: claude_resp.usage.input_tokens,
                output_tokens: claude_resp.usage.output_tokens,
            },
            reasoning: None,
        })
    }

    fn name(&self) -> &str {
        "claude"
    }
}
