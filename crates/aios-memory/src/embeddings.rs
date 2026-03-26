//! Embedding Store - Vector storage for semantic search

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

/// Embedding entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingEntry {
    pub id: Uuid,
    pub content: String,
    pub vector: Vec<f32>,
    pub metadata: HashMap<String, String>,
}

/// Embedding store for semantic search
pub struct EmbeddingStore {
    entries: HashMap<Uuid, EmbeddingEntry>,
    storage_path: PathBuf,
}

impl EmbeddingStore {
    /// Create a new embedding store
    pub fn new(path: &PathBuf) -> Result<Self> {
        let storage_path = path.join("embeddings");
        std::fs::create_dir_all(&storage_path)?;

        Ok(Self {
            entries: HashMap::new(),
            storage_path,
        })
    }

    /// Add an embedding
    pub fn add(&mut self, content: &str, vector: Vec<f32>) -> Result<Uuid> {
        let id = Uuid::new_v4();
        let entry = EmbeddingEntry {
            id,
            content: content.to_string(),
            vector,
            metadata: HashMap::new(),
        };
        self.entries.insert(id, entry);
        Ok(id)
    }

    /// Search for similar embeddings (cosine similarity)
    pub fn search(&self, query: &[f32], limit: usize) -> Vec<(Uuid, f32)> {
        let mut results: Vec<_> = self
            .entries
            .iter()
            .map(|(id, entry)| {
                let similarity = cosine_similarity(query, &entry.vector);
                (*id, similarity)
            })
            .collect();

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit);
        results
    }

    /// Get embedding by ID
    pub fn get(&self, id: Uuid) -> Option<&EmbeddingEntry> {
        self.entries.get(&id)
    }
}

/// Calculate cosine similarity between two vectors
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }

    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let magnitude_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let magnitude_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if magnitude_a == 0.0 || magnitude_b == 0.0 {
        return 0.0;
    }

    dot_product / (magnitude_a * magnitude_b)
}
