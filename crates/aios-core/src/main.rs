//! AI-OS Core - Main entry point

use anyhow::Result;
use tracing::info;

mod config;
mod error;

pub use error::AIOSError;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    info!("Starting AI-OS...");

    // Run the main application
    aios_core::run().await
}
