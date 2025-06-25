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

pub struct PositionManager {
    positions: HashMap<String, Position>,
}

impl PositionManager {
    pub fn new() -> Self {
        Self {
            positions: HashMao::new(),
        }
    } 

    pub fn update_position(&mut self, symbol: String, side: Side, quantity:f64, fill_price: f64) {
       let position =  self.positions.entry(symbol.clone()).or_insert_with(|| Position::new(symbol.clone()));

       match side {
        Side::Buy => {

            if position.quantity < 0.0 {
                let quantity_to_close = quantity.min(-position.quantity);
                let remaining_buy_quantity = quantity - quantity_to_close;

                position.pnl_realized += (position.avg_cost - fill_price * remaining_buy_quantity) / position.quantity;
            }
        }
       }
    }
}