//! AI-OS Memory Store - Context persistence and conversation history

mod conversation;
mod embeddings;

pub use conversation::ConversationStore;
pub use embeddings::EmbeddingStore;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Memory Store - Main entry point for memory operations
pub struct MemoryStore {
    conversations: ConversationStore,
    embeddings: EmbeddingStore,
    db_path: PathBuf,
}

impl MemoryStore {
    /// Create a new memory store
    pub fn new() -> Result<Self> {
        let db_path = PathBuf::from(".aios/data");

        Ok(Self {
            conversations: ConversationStore::new(&db_path)?,
            embeddings: EmbeddingStore::new(&db_path)?,
            db_path,
        })
    }

    /// Get conversation store
    pub fn conversations(&self) -> &ConversationStore {
        &self.conversations
    }

    /// Get embedding store
    pub fn embeddings(&self) -> &EmbeddingStore {
        &self.embeddings
    }

    /// Save a message to conversation history
    pub fn save_message(&self, session_id: Uuid, role: &str, content: &str) -> Result<()> {
        self.conversations.add_message(session_id, role, content)
    }

    /// Get conversation history
    pub fn get_conversation(&self, session_id: Uuid) -> Result<Vec<ConversationMessage>> {
        self.conversations.get_conversation(session_id)
    }
}

impl Default for MemoryStore {
    fn default() -> Self {
        Self::new().expect("Failed to create memory store")
    }
}

/// A message in a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessage {
    pub id: Uuid,
    pub session_id: Uuid,
    pub role: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}
