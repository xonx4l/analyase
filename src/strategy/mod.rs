pub mod mean_reversion;

use crate::config::StrategyConfig;
use crate::data_model::Order; 
use tokio::sync::mpsc;
use tracing::{info, debug}; 
use anyhow::Result;

#[derive(Debug, Clone)]
pub enum StrategyControl {
    Start,
    Stop,
    UpdateParams(StrategyConfig),
}

pub async fn run_strategy_engine(
    mut config: StrategyConfig,
    mut control_rx: mpsc::UnboundedReceiver<StrategyControl>,
    _order_tx: mpsc::UnboundedSender<Order>, 
) -> Result<()> {
    info!("Strategy Engine started. Config: {:?}", config);

    let mut is_running = config.enabled;

    loop {
        tokio::select! {
            Some(control_msg) = control_rx.recv() => {
                match control_msg {
                    StrategyControl::Start => {
                        is_running = true;
                        info!("Strategy Engine received START command.");
                    },
                    StrategyControl::Stop => {
                        is_running = false;
                        info!("Strategy Engine received STOP command.");
                    },
                    StrategyControl::UpdateParams(new_config) => {
                        config = new_config;
                        info!("Strategy Engine parameters updated: {:?}", config);
                    },
                }
            }
            _ = tokio::signal::ctrl_c() => {
                info!("Strategy Engine received Ctrl-C, shutting down.");
                break;
            }
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(5)) => {
                if is_running {
                    debug!("Strategy heartbeat (replace with actual algo logic)");
                }
            }
        }
    }
    Ok(())
}