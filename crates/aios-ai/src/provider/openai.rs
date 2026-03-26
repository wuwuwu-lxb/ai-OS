//! OpenAI API provider

use super::{AIProvider, ProviderError};
use crate::gateway::{AIRequest, AIResponse, Message, Model, Role, Usage};
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";

pub struct OpenAIProvider {
    client: Client,
    api_key: String,
    model: String,
}

impl OpenAIProvider {
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
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Serialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    id: String,
    model: String,
    choices: Vec<OpenAIChoice>,
    usage: OpenAIUsage,
}

#[derive(Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessageContent,
}

#[derive(Deserialize)]
struct OpenAIMessageContent {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[async_trait]
impl AIProvider for OpenAIProvider {
    async fn complete(&self, req: &AIRequest) -> Result<AIResponse> {
        let Model::OpenAI { version } = &req.model else {
            anyhow::bail!("Invalid model for OpenAI provider");
        };

        // Convert messages
        let messages: Vec<OpenAIMessage> = req
            .messages
            .iter()
            .map(|m| {
                let role = match m.role {
                    Role::User => "user",
                    Role::Assistant => "assistant",
                    Role::System => "system",
                };
                OpenAIMessage {
                    role: role.to_string(),
                    content: m.content.clone(),
                }
            })
            .collect();

        let openai_req = OpenAIRequest {
            model: version.clone(),
            messages,
            temperature: req.temperature,
            max_tokens: req.max_tokens,
        };

        debug!("Sending request to OpenAI API");

        let response = self
            .client
            .post(OPENAI_API_URL)
            .header("authorization", format!("Bearer {}", self.api_key))
            .header("content-type", "application/json")
            .json(&openai_req)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            info!("OpenAI API error: {} - {}", status, error_text);

            if status.as_u16() == 401 {
                return Err(ProviderError::AuthError("Invalid API key".to_string()).into());
            } else if status.as_u16() == 429 {
                return Err(ProviderError::RateLimitError("Rate limit exceeded".to_string()).into());
            }
            return Err(ProviderError::ApiError(format!("{}: {}", status, error_text)).into());
        }

        let openai_resp: OpenAIResponse = response.json().await
            .context("Failed to parse OpenAI response")?;

        let content = openai_resp
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        Ok(AIResponse {
            content,
            model: openai_resp.model,
            usage: Usage {
                input_tokens: openai_resp.usage.prompt_tokens,
                output_tokens: openai_resp.usage.completion_tokens,
            },
            reasoning: None,
        })
    }

    fn name(&self) -> &str {
        "openai"
    }
}
