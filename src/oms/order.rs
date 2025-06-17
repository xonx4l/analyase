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