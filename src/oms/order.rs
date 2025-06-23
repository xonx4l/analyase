use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::data_model::{OrderType, Side, TimeInForce, Order as UiOrder};

#[derive(Debug, Clone, PartialEq, Serialize , Deserialize )]
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

#[derive(Debug, Clone, PartialEq, Serialize , Deserialize )]
pub struct FullOrder {
    pub order: UiOrder,
    pub current_state: OrderState,
    pub filled_quantity:f64,
    pub avg_fill_price:f64,
    pub last_fill_price: Option<f64>,
    pub last_fill_quantity: Option<f64>,
    pub last_fill_time: Option<DateTime<Utc>>,
    pub exchange_order_id: Option<String>,
    pub rejection_reason: Option<String>,
}

impl FullOrder {
    pub fn from(ui_order: Ui_Order) -> Self {
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
}