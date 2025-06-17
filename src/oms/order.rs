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