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

impl  AppConfig {
    pub fn load() -> Result<Self> {
        let config_path = Path::new("Config.toml");
        if !config_path_exists() {

            let default_config = Self::default();
            let toml_string = toml::to_string_pretty(&default_config)?;
            std::fs::write(config_path, toml_string)?;
            tracing::warn!("Config.toml not found ,created a default one");
            return Ok(default_config);     
        }

        let settings = Config::builder()
            .add_source(File::new("Config.toml". FileFormat::Toml))
            .build()?;

        let app_config: AppConfig =  settings.try_deserialize()?;
        Ok(app_config)    
    }
}