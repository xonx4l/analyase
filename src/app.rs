
use egui::{CentralPanel, Context, TopBottomPanel, RichText, Color32, Layout, ScrollArea};
use eframe::App;
use tokio::sync::mpsc;
use tracing::error; 

use egui_plot::{Line, Plot, Legend}; 

use crate::data_model::{MarketDataUpdate, Order, OrderType, Side, TimeInForce};
use crate::oms::OmsUpdate;
use crate::strategy::StrategyControl;

pub struct AlgoApp {
    
    market_data_rx: mpsc::UnboundedReceiver<MarketDataUpdate>,
    oms_rx: mpsc::UnboundedReceiver<OmsUpdate>,

    
    order_tx: mpsc::UnboundedSender<Order>,
    strategy_tx: mpsc::UnboundedSender<StrategyControl>,

    
    last_market_data: Option<MarketDataUpdate>,
    orders: Vec<Order>,
    oms_log: Vec<String>, 
    app_log: Vec<String>,

    input_symbol: String,
    input_price: String,
    input_quantity: String,
    input_side: Side,
    input_order_type: OrderType,
    input_tif: TimeInForce,

    strategy_status: String,

    price_history: Vec<[f64; 2]>,
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

    fn handle_market_data_updates(&mut self) {
        while let Ok(update) = self.market_data_rx.try_recv() {
            self.last_market_data = Some(update.clone());

            if let Some(last_price) = update.last_price {
                self.price_history.push([
                    update.timestamp.timestamp_millis() as f64,
                    last_price,
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
                    self.log_message(format!("Order Created: {:?} {}", order.side, order.symbol));
                    self.orders.push(order); 
                }
                OmsUpdate::OrderStateChange { order_id, new_state, .. } => {
                    if let Some(order) = self.orders.iter_mut().find(|o| o.order_id == order_id) {
                        order.state = new_state.clone();
                        self.log_message(format!("Order {} State Changed: {:?}", order_id, new_state));
                    }
                }
                OmsUpdate::PositionUpdate(pos) => {
                    self.log_message(format!("Position Update: {} {} @ {:.2}", pos.symbol, pos.quantity, pos.avg_cost));
                }
            }
        }
    }

    fn log_message(&mut self, msg: String) {
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

    fn render_order_entry_panel(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.heading("Manual Order Entry");
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Symbol:");
                ui.text_edit_singleline(&mut self.input_symbol);
            });
            ui.horizontal(|ui| {
                ui.label("Price:");
                ui.text_edit_singleline(&mut self.input_price);
            });
            ui.horizontal(|ui| {
                ui.label("Quantity:");
                ui.text_edit_singleline(&mut self.input_quantity);
            });
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Side:");
                ui.radio_value(&mut self.input_side, Side::Buy, "Buy");
                ui.radio_value(&mut self.input_side, Side::Sell, "Sell");
            });
            ui.horizontal(|ui| {
                ui.label("Type:");
                ui.radio_value(&mut self.input_order_type, OrderType::Limit, "Limit");
                ui.radio_value(&mut self.input_order_type, OrderType::Market, "Market");
            });
            ui.horizontal(|ui| {
                ui.label("TIF:");
                ui.radio_value(&mut self.input_tif, TimeInForce::GTC, "GTC");
                ui.radio_value(&mut self.input_tif, TimeInForce::IOC, "IOC");
            });
            ui.add_space(10.0);

            if ui.button(RichText::new("Place Order").strong().color(Color32::WHITE)).clicked() {
                let quantity = self.input_quantity.parse::<f64>().unwrap_or_default();
                let price = self.input_price.parse::<f64>().ok();

                if quantity > 0.0 && !self.input_symbol.is_empty() {
                    let new_order = Order::new(
                        self.input_symbol.clone(),
                        self.input_side.clone(),
                        self.input_order_type.clone(),
                        quantity,
                        price,
                        self.input_tif.clone(),
                    );
                    if let Err(e) = self.order_tx.send(new_order) {
                        error!("Failed to send order to OMS: {}", e); 
                        self.log_message(format!("Failed to place order: {}", e));
                    } else {
                        self.log_message(format!("Order placed: {:?} {} @ {}", self.input_side, quantity, self.input_symbol));
                    }
                } else {
                    self.log_message("Invalid order input: Quantity must be > 0 and Symbol cannot be empty.".to_string());
                }
            }
        });
    }

    fn render_orders_table(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.heading("Current Orders");
            ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                egui::Grid::new("orders_grid")
                    .num_columns(8) 
                    .spacing([20.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.strong("ID");
                        ui.strong("Symbol");
                        ui.strong("Side");
                        ui.strong("Type");
                        ui.strong("Qty");
                        ui.strong("Price");
                        ui.strong("TIF");
                        ui.strong("Status");
                        ui.end_row();

                        for order in &self.orders {
                            ui.label(order.order_id.simple().to_string());
                            ui.label(&order.symbol);
                            ui.label(format!("{:?}", order.side));
                            ui.label(format!("{:?}", order.order_type));
                            ui.label(format!("{:.2}", order.quantity));
                            ui.label(format!("{:.2}", order.price.unwrap_or(0.0)));
                            ui.label(format!("{:?}", order.tif));
                            ui.label(format!("{:?}", order.state));
                            ui.end_row();
                        }
                        if self.orders.is_empty() {
                            ui.label("No active orders.");
                            ui.end_row();
                        }
                    });
            });
        });
    }

    fn render_app_log_panel(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.heading("Application Log");
            ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                ui.vertical(|ui| {
                    for entry in self.app_log.iter().rev() { 
                        ui.label(entry);
                    }
                });
            });
        });
    }
}

impl App for AlgoApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.handle_market_data_updates();
        self.handle_oms_updates();

        ctx.request_repaint();

        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(5.0);
            egui::menu::bar(ui, |ui| {
                ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.heading(RichText::new("Rust Algo System").color(Color32::LIGHT_BLUE).strong());
                });

                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("Strategy Status: {}", self.strategy_status));
                    if ui.button("Start Strategy").clicked() {
                        self.log_message("Strategy start button clicked.".to_string());
                        if let Err(e) = self.strategy_tx.send(StrategyControl::Start) {
                            error!("Failed to send Start Strategy command: {}", e); 
                            self.log_message(format!("Error: {}", e));
                        }
                    }
                    if ui.button("Stop Strategy").clicked() {
                        self.log_message("Strategy stop button clicked.".to_string());
                        if let Err(e) = self.strategy_tx.send(StrategyControl::Stop) {
                            error!("Failed to send Stop Strategy command: {}", e); 
                            self.log_message(format!("Error: {}", e));
                        }
                    }
                });
            });
            ui.add_space(5.0);
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Dashboard");
            ui.add_space(10.0);

            egui::Grid::new("main_dashboard_grid")
                .num_columns(2)
                .spacing([20.0, 20.0])
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        self.render_market_data_panel(ui);
                        ui.add_space(10.0);
                        self.render_order_entry_panel(ui);
                    });
  
                    ui.vertical(|ui| {
                        self.render_orders_table(ui);
                        ui.add_space(10.0);
                        self.render_app_log_panel(ui);
                    });
                    ui.end_row();
                });
        });
    }
}