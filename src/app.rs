use egui::{CentralPanel, Context, TopBottomPanel, RichText, Color32, Layout, ScrollArea};
use eframe::App;
use tokio::sync::mpsc;
use tracing::error; 

use egui_plot::{Line, Plot, Legend}; 
                                     
pub struct AlgoApp {
    
    //Communication channels to recieve updates from backend
    market_data_rx: mpsc::UnboundedReciever<MarketDataUpdate>,
    oms_rx: mpsc::UnboundedReciever<OmsUpdate>,

    // Communication channels to send commands to backend 
    order_tx: mpsc::UnboundedSender<Order>,
    strategy_tx: mpsc::UnboundedSender<StrategyControl>,

    // UI State 
    last_market_data: Option<MarketDataUpdate>,
    orders: Vec<Order>,
    oms_log: Vec<String>,
    app_log: Vec<String>,

    
}

impl AlgoApp {
    pub fn new(
        market_data_rx: mpsc::UnboundedReceiver<MarketDataUpdate>,
        oms_rx: mpsc::UnboundedReceiver<OmsUpdate>,
        order_tx: mpsc::UnboundedSender<Order>,
        strategy_tx: mpsc::UnboundedSender<StrategyControl>,
    ) -> Self {
        Self {
            market_data_rx,
            oms_rx,
            order_tx,
            strategy_tx,
            last_market_data: None,
            orders: Vec::new(),
            oms_log: Vec::new(),
            app_log: Vec::new(),
        }

    }
}
