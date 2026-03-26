//! Conversation Store - Conversation history management

use super::ConversationMessage;
use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use std::cell::RefCell;
use uuid::Uuid;
use chrono::Utc;

/// In-memory conversation store (sled integration ready)
pub struct ConversationStore {
    conversations: RefCell<HashMap<Uuid, Vec<ConversationMessage>>>,
    storage_path: PathBuf,
}

impl ConversationStore {
    /// Create a new conversation store
    pub fn new(path: &PathBuf) -> Result<Self> {
        let storage_path = path.join("conversations");
        std::fs::create_dir_all(&storage_path)?;

        Ok(Self {
            conversations: RefCell::new(HashMap::new()),
            storage_path,
        })
    }

    /// Add a message to a conversation
    pub fn add_message(&self, session_id: Uuid, role: &str, content: &str) -> Result<()> {
        let message = ConversationMessage {
            id: Uuid::new_v4(),
            session_id,
            role: role.to_string(),
            content: content.to_string(),
            timestamp: Utc::now(),
        };

        self.conversations
            .borrow_mut()
            .entry(session_id)
            .or_insert_with(Vec::new)
            .push(message);

        Ok(())
    }

    /// Get all messages in a conversation
    pub fn get_conversation(&self, session_id: Uuid) -> Result<Vec<ConversationMessage>> {
        Ok(self
            .conversations
            .borrow()
            .get(&session_id)
            .cloned()
            .unwrap_or_default())
    }

    /// List all sessions
    pub fn list_sessions(&self) -> Vec<Uuid> {
        self.conversations.borrow().keys().cloned().collect()
    }

    /// Delete a conversation
    pub fn delete_conversation(&self, session_id: Uuid) -> Option<Vec<ConversationMessage>> {
        self.conversations.borrow_mut().remove(&session_id)
    }
}
