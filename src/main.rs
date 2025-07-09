mod app;
mod config;
mod data_model;
mod market_data;
mod oms;
mod strategy;
mod utils;

use app::AlgoApp;
use tokio::sync::mpsc;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    utils::init_logging()?;
    info!("Application starting (UI only at this stage)...");

    let (_md_tx, md_rx) = mpsc::unbounded_channel(); 
    let (_oms_tx, oms_rx) = mpsc::unbounded_channel(); 
    let (ui_order_tx, _ui_order_rx) = mpsc::unbounded_channel(); 
    let (ui_strategy_tx, _ui_strategy_rx) = mpsc::unbounded_channel(); 


    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
          
        ..Default::default()
    };

    let app_state = AlgoApp::new(md_rx, oms_rx, ui_order_tx, ui_strategy_tx);

    eframe::run_native(
        "Analyase",
        native_options,
        Box::new(|_cc| Box::new(app_state)),
    )
    .map_err(|e| anyhow::anyhow!("eframe error: {}", e))?;

    info!("Application shut down.");

    Ok(())
}