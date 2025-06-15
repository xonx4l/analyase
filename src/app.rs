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

    //Input fields for manual data entry 
    input_symbol: String,
    input_price: String,
    input_quantity: String,
    input_side: Side,
    input_order_type: OrderType,
    input_tif: TimeInForce,

    strategy_status: String,

    price_history: Vec<[f64;2]>,

    
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
            input_symbol: "BTCUSD".to_string(),
            input_price: "0.0".to_string(),
            input_quantity: "1.0".to_string(),
            input_side: Side::Buy,
            input_order_type: OrderType::Limit,
            input_tif: TimeInForce::GTC,
            strategy_status: "Inactive".to_string(),
            price_history: Vec::new(),
        }

    }
}

fn handle_market_data_updates(&mut self)  {
    while let Ok(update) = self.market_data_rx.try_recv() {
        self.last_market_data = Some(update.Clone());

        if let Some(last_price) = update.last_price {
               self.price_history.push([
                update.timestamp.timestamp_millis() as f64,
                last price,
               ]);

               if self.price_history.len() > 1000 {
                  self.price_history.remove(0);
               }
        }
    }

}

fn handle_oms_updates(&mut self) {
    while let Ok(update) = self.oms_rx.try_recv() {
        match update {
            OmsUpdate::OrderCreated(order) => {
                self.log_message(format!("Order created {;?} {}, order.side, order.symbol "));
                self.orders.push.(order);
            }
            OmsUpdate::OrderStateChange{ order_id, new_state,...} => {
                if let Some(order) = self.orders.iter_mut().find(|o| o.order_id == order_id) {
                    order.state = new_state.clone();
                    self.log_message(format!("Order {} State Changed {:?} ", order_id, new_state));
                }
            }
            OmsUpdate::PositionUpdate(pos) => {
                self.log_message(format!("Position Update: {} {} @ {:.2}",pos.symbol, pos.quantity, pos.avg_cost));
            }
        }
    }
}

fn log_message(&mut self , msg: String) {
    self.app_log.push(msg);
    if self.app_log.len() > 200 {
        self.app_log.remove(0);
    }
}

fn render_market_data_panel(&mut self, ui: &mut egui::Ui) {
    ui.group(|ui| {
        ui.heading("Market Data & Chart");

        if let Some(md) = &self.last_market_data {
            ui.label(format!("Symbol: {}", md.symbol));
            if let Some(last_price) = md.last_price {
                ui.label(format!("Last Price: {:.2}", last_price));
            }
            ui.label(format!("Time: {}", md.timestamp.format("%H:%M:%S")));
        } else {
            ui.label("Waiting for market data...");
        }

        ui.add_space(10.0);

        if !self.price_history.is_empty() {
            Plot::new("price_chart")
                .height(300.0) 
                .data_aspect(1.0) 
                .show_axes([true, true])
                .show_x(true)
                .show_y(true)
                .label_formatter(|name, value| {
                    if name == "X" {
                        let timestamp_ms = value.x as i64;
                        let datetime = chrono::DateTime::from_timestamp_millis(timestamp_ms)
                            .unwrap_or_else(|| chrono::Utc::now()); 
                        format!("{}", datetime.format("%H:%M:%S"))
                    } else {
                        format!("{}: {:.2}", name, value.y)
                    }
                })
                .legend(Legend::default()) 
                .show(ui, |plot_ui| {
                    let line = Line::new(self.price_history.clone())
                        .name("Last Price")
                        .color(Color32::LIGHT_GREEN);
                    plot_ui.line(line);
                });
        } else {
            ui.label("No price data for chart yet.");
        }
    });
}

fn render_market_entry_panel(&mut self, ui: &mut egui::Ui) {
   ui.group(|ui| {
    ui.heading("Manual order entry");
    ui.add_space(5.0);

    ui.horizontal(|ui|{
        ui.label("Symbol:");
        ui.text_edit_single_line(&mut self.input_symbol)
    });

    ui.horizontal(|ui| {
        ui.label("Price:");
        ui.text_edit_single_line(&mut self.input_price)
    });
    ui.horizontal(|ui {
        ui.label("Quantity:");
        ui.text_edit_single_line(&mut self.input_quantity)
    });
    ui_add_space(5.0);
   })
}

fn render_order_table(&mut self , ui: &mut egui::Ui){
    ui.group(|ui|{
        ui.heading("Current Orders");
    })
}