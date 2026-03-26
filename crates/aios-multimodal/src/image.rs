//! Image processor

pub struct ImageProcessor;

impl ImageProcessor {
    pub fn new() -> Self {
        Self
    }

    pub fn process(&self, data: &[u8]) -> Result<String, String> {
        // Placeholder: encode as base64 for API
        use base64::{Engine as _, engine::general_purpose};
        Ok(general_purpose::STANDARD.encode(data))
    }
}
