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
    pub enabled: bool,
    pub name: String,
    pub mean_reversion_threshold: f64,
}

pub struct AppConfig {
    pub market_data: MarketDataConfig,
    pub strategy: StrategyConfig,
}

pub struct DataManage{
    
}