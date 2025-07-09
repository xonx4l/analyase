use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid; 

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,

}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TimeInForce {
    GTC, 
    IOC,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candlestick {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDataUpdate {
    pub symbol: String,
    pub timestamp: DateTime<Utc>,
    pub bid_price: Option<f64>,
    pub bid_quantity: Option<f64>,
    pub ask_price: Option<f64>,
    pub ask_quantity: Option<f64>,
    pub last_price: Option<f64>,
    pub last_quantity: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub order_id: Uuid,
    pub client_order_id: String, 
    pub symbol: String,
    pub side: Side,
    pub order_type: OrderType,
    pub quantity: f64,
    pub price: Option<f64>, 
    pub tif: TimeInForce,
    pub placed_at: DateTime<Utc>,
    pub state: crate::oms::OrderState, 
}

impl Order {
    pub fn new(
        symbol: String,
        side: Side,
        order_type: OrderType,
        quantity: f64,
        price: Option<f64>,
        tif: TimeInForce,
    ) -> Self {
        let order_id = Uuid::new_v4();
        let client_order_id = format!("cl_{}", order_id.to_string()); 
        Self {
            order_id,
            client_order_id,
            symbol,
            side,
            order_type,
            quantity,
            price,
            tif,
            placed_at: Utc::now(),
            state: crate::oms::OrderState::New, 
        }
    }
}