pub mod websocket;
use crate::config::MarketDataConfig;
use crate::data_model::MarketDataUpdate;
use tokio::sync::mpsc;
use tracing::info; 

pub async fn run_market_data_handler(
    _config: MarketDataConfig,
    _md_tx: mpsc::UnboundedSender<MarketDataUpdate>,
) -> anyhow::Result<()> {
    info!("Market Data Handler started (dummy).");
    Ok(())
}