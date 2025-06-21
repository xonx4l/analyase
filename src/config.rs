use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MarketDataConfig{
    pub websocket_url: String,
    pub symbols: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StrategyConfig {
    
}