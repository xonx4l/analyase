
use tokio::sync::mpsc;
use crate::data_model::{MarketDataUpdate, Order};
use crate::oms::OmsUpdate;
use crate::strategy::StrategyControl;

pub struct AppChannels {
    pub md_to_ui_tx: mpsc::UnboundedSender<MarketDataUpdate>,
    pub md_to_ui_rx: mpsc::UnboundedReceiver<MarketDataUpdate>, 
    pub oms_to_ui_rx: mpsc::UnboundedReceiver<OmsUpdate>,

    pub ui_to_oms_tx: mpsc::UnboundedSender<Order>,
    pub ui_to_oms_rx: mpsc::UnboundedReceiver<Order>,

    pub ui_to_strategy_tx: mpsc::UnboundedSender<StrategyControl>,
    pub ui_to_strategy_rx: mpsc::UnboundedReceiver<StrategyControl>,

    pub md_to_strategy_tx: mpsc::UnboundedSender<MarketDataUpdate>,
    pub md_to_strategy_rx: mpsc::UnboundedReceiver<MarketDataUpdate>,
}

impl AppChannels {
    pub fn new() -> Self {
        let (md_to_ui_tx, md_to_ui_rx) = mpsc::unbounded_channel();
        let (oms_to_ui_tx, oms_to_ui_rx) = mpsc::unbounded_channel();
        let (ui_to_oms_tx, ui_to_oms_rx) = mpsc::unbounded_channel();
        let (ui_to_strategy_tx, ui_to_strategy_rx) = mpsc::unbounded_channel();
        let (md_to_strategy_tx, md_to_strategy_rx) = mpsc::unbounded_channel();

        Self {
            md_to_ui_tx, md_to_ui_rx,
            oms_to_ui_tx, oms_to_ui_rx,
            ui_to_oms_tx, ui_to_oms_rx,
            ui_to_strategy_tx, ui_to_strategy_rx,
            md_to_strategy_tx, md_to_strategy_rx,
        }
    }
}