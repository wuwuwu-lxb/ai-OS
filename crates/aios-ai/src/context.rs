//! Context Manager - Conversation history and context window management

use crate::gateway::{Message, Role};
use chrono::Utc;
use std::collections::VecDeque;

/// Context manager for handling conversation history
pub struct ContextManager {
    /// Rolling context window
    messages: VecDeque<Message>,
    /// Maximum tokens in context window (approximate)
    max_tokens: usize,
    /// Estimated tokens per message (rough average)
    tokens_per_message: usize,
}

impl ContextManager {
    /// Create a new context manager
    pub fn new(max_tokens: usize) -> Self {
        Self {
            messages: VecDeque::new(),
            max_tokens,
            tokens_per_message: 50, // Rough estimate for average message
        }
    }

    /// Add a message to the context
    pub fn add_message(&mut self, role: Role, content: String) {
        let message = Message {
            role,
            content,
            timestamp: Utc::now(),
        };

        self.messages.push_back(message);
        self.evict_if_needed();
    }

    /// Get all messages in the context
    pub fn get_messages(&self) -> Vec<Message> {
        self.messages.iter().cloned().collect()
    }

    /// Clear the context
    pub fn clear(&mut self) {
        self.messages.clear();
    }

    /// Get estimated token count
    pub fn estimated_tokens(&self) -> usize {
        self.messages.len() * self.tokens_per_message
    }

    /// Evict oldest messages if context is too large
    fn evict_if_needed(&mut self) {
        while self.estimated_tokens() > self.max_tokens && self.messages.len() > 1 {
            self.messages.pop_front();
        }
    }
}

impl Default for ContextManager {
    fn default() -> Self {
        Self::new(128_000) // Default 128K tokens
    }
}
