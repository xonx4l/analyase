// src/utils/logger.rs
use tracing_subscriber::{EnvFilter, fmt}; 
use anyhow::Result;

pub fn init_logging() -> Result<()> {
    fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env()) 
        .init();
    Ok(())
}