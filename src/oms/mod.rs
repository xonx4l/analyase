pub mod order;
pub mod positions;

use tokio::sync::mpsc;
use tracing::{info, error};
use std::collection::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use anyhow::Result;

#[derive(Debug, Clone)]
pub enum OmsUpdate{
    OrderCreated(Order),
    OrderStateChange{
        order_id: uuid::Uuid,
        new_state; OrderState,
        timestamp: chrono::DateTime::<chrono::Utc>,
    },
    PositionUpdate(Position),
}

pub async fn run_oms(
    mut ui_order_rx: mpsc::UnboundedReceiver<Order>,
    oms_ui_tx: mpsc::UnboundedSender<OmsUpdate>,
) -> Result<()> {
   info!("Order Management System (OMS) started.");

   let orders: Arc<RwLock<HashMap<uuid::Uuid, FullOrder>>> = Arc::new(RwLock::new(HashMap::new()));
   let position_manager = Arc::new(RwLock::new(PositionManager::new()));

}