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

    
}
