//! Input normalizer - unified input processing pipeline

use super::{InputMessage, Modality};

pub struct InputNormalizer;

impl InputNormalizer {
    pub fn new() -> Self {
        Self
    }

    /// Detect input modality
    pub fn detect_modality(&self, input: &str) -> Modality {
        // Simple detection based on content
        if input.starts_with("data:image") {
            Modality::Image
        } else if input.contains("```") {
            Modality::Code
        } else {
            Modality::Text
        }
    }

    /// Normalize input to unified format
    pub fn normalize(&self, input: &str) -> InputMessage {
        let modality = self.detect_modality(input);

        InputMessage {
            modality,
            content: input.to_string(),
            metadata: std::collections::HashMap::new(),
        }
    }
}
