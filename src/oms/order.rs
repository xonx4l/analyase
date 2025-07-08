// src/oms/order.rs
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid; // Keep this, as Uuid is used in struct definitions

// Removed unused imports: OrderType, Side, TimeInForce are used within UiOrder struct,
// so they're implicitly "used" via the UiOrder definition.
use crate::data_model::Order as UiOrder;


/// Represents the possible states of an order in the OMS.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderState {
    New,
    PendingNew,
    Open,
    Filled,
    PartialFill,
    Canceled,
    PendingCancel,
    Rejected,
    Expired,
}

/// Represents the full, internal state of an order within the OMS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullOrder {
    pub order: UiOrder,
    pub current_state: OrderState,
    pub filled_quantity: f64,
    pub avg_fill_price: f64,
    pub last_fill_price: Option<f64>,
    pub last_fill_quantity: Option<f64>,
    pub last_fill_time: Option<DateTime<Utc>>,
    pub exchange_order_id: Option<String>,
    pub rejection_reason: Option<String>,
}

impl FullOrder {
    pub fn from(ui_order: UiOrder) -> Self {
        Self {
            order: ui_order.clone(),
            current_state: ui_order.state,
            filled_quantity: 0.0,
            avg_fill_price: 0.0,
            last_fill_price: None,
            last_fill_quantity: None,
            last_fill_time: None,
            exchange_order_id: None,
            rejection_reason: None,
        }
    }

    pub fn update_state(&mut self, new_state: OrderState) {
        self.current_state = new_state;
    }
}