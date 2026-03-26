//! AI Response Cache

use crate::gateway::AIResponse;
use std::collections::HashMap;
use std::sync::RwLock;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// Simple content-addressable cache for AI responses
pub struct AICache {
    store: RwLock<HashMap<u64, AIResponse>>,
    max_size: usize,
}

impl AICache {
    /// Create a new cache with specified max size
    pub fn new() -> Self {
        Self {
            store: RwLock::new(HashMap::new()),
            max_size: 1000,
        }
    }

    /// Get a cached response for the given prompt
    pub fn get(&self, prompt: &str) -> Option<AIResponse> {
        let hash = self.hash(prompt);
        let store = self.store.read().unwrap();
        store.get(&hash).cloned()
    }

    /// Insert a response into the cache
    pub fn insert(&self, prompt: String, response: AIResponse) {
        // Check size and evict if needed
        {
            let mut store = self.store.write().unwrap();
            if store.len() >= self.max_size {
                // Simple eviction: clear half the cache
                let keys: Vec<_> = store.keys().take(self.max_size / 2).cloned().collect();
                for key in keys {
                    store.remove(&key);
                }
            }
        }

        let hash = self.hash(&prompt);
        let mut store = self.store.write().unwrap();
        store.insert(hash, response);
    }

    /// Clear the cache
    pub fn clear(&self) {
        let mut store = self.store.write().unwrap();
        store.clear();
    }

    /// Hash a string to a u64
    fn hash(&self, s: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    }
}

impl Default for AICache {
    fn default() -> Self {
        Self::new()
    }
}
