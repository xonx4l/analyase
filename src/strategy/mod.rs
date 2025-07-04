pub mod mean_reversion;

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
    info!("Strategy Engine Started. Config: {:?}", config);

    let mut is_running =  config.enabled;

    loop{
        tokio::select! {

        }
    }
}
