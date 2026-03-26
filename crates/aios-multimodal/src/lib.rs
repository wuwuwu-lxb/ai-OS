//! Multi-Modal Input Processing
//!
//! Handles text, voice, image, and code inputs

mod text;
mod image;
mod code;
mod normalizer;

pub use text::TextProcessor;
pub use image::ImageProcessor;
pub use code::CodeProcessor;
pub use normalizer::InputNormalizer;

/// Input modality types
#[derive(Debug, Clone)]
pub enum Modality {
    Text,
    Voice,
    Image,
    Code,
}

/// Normalized input message
#[derive(Debug)]
pub struct InputMessage {
    pub modality: Modality,
    pub content: String,
    pub metadata: std::collections::HashMap<String, String>,
}
