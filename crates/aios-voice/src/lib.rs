//! Voice processing for AI-OS

mod tts;

pub use tts::TTSEngine;

/// Voice output
pub trait VoiceOutput {
    fn speak(&self, text: &str) -> Result<(), String>;
    fn stop(&self) -> Result<(), String>;
}
