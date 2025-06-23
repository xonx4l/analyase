use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct Position{
    pub symbol: String,
    pub quantity: f64.
    pub avg_cost: f64,
    pub last_price: f64,
    pub pnl_unrealized: f64,
    pub pnl_realized: f64,
}

impl Position {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            quantity:0.0,
            avg_cost:0.0,
            last_price:0.0,
            pnl_unrealized:0.0,
            pnl_realized:0.0,
        }
    }

    pub fn update_unrealized_pnl(&mut self) {
        if self.quantity !=0.0 && self.last_price > 0.0 {
           self.pnl_unrealized = (self.last_price - self.avg_cost) * self.quantity;
        } else {
            self.pnl_unrealized = 0.0;
        }
    }
}
