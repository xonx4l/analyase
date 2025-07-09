pub mod order;
pub mod position;

use tokio::sync::mpsc;
use tracing::{info, error}; 
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use anyhow::Result;

use crate::data_model::Order;
pub use crate::oms::order::{OrderState, FullOrder};
use crate::oms::position::PositionManager;
use crate::oms::position::Position;

#[derive(Debug, Clone)]
pub enum OmsUpdate {
    OrderCreated(Order), 
    OrderStateChange {
        order_id: uuid::Uuid, 
        new_state: OrderState,
        timestamp: chrono::DateTime<chrono::Utc>,
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

    loop {
        tokio::select! {
            Some(mut order) = ui_order_rx.recv() => {
                info!("OMS received order: {:?}", order);
                let order_id = order.order_id;
                order.state = OrderState::PendingNew; 
                let oms_order = FullOrder::from(order.clone()); 
                orders.write().insert(order_id, oms_order);

                if let Err(e) = oms_ui_tx.send(OmsUpdate::OrderCreated(order.clone())) {
                    error!("Failed to send OrderCreated update to UI: {}", e);
                }

                let oms_ui_tx_clone = oms_ui_tx.clone();
                let orders_clone = orders.clone();
                let position_manager_clone = position_manager.clone();

                tokio::spawn(async move {
                    tokio::time::sleep(tokio::time::Duration::from_millis(100 + rand::random::<u64>() % 200)).await;

                    let mut orders_guard = orders_clone.write();
                    if let Some(order_to_execute) = orders_guard.get_mut(&order_id) {
                        order_to_execute.current_state = OrderState::Filled; 
                        order_to_execute.filled_quantity = order_to_execute.order.quantity;
                        order_to_execute.avg_fill_price = order_to_execute.order.price.unwrap_or(0.0); 

                        info!("Simulating fill for order {}: {:?}", order_id, order_to_execute.current_state); 

                        position_manager_clone.write().update_position(
                            order_to_execute.order.symbol.clone(),
                            order_to_execute.order.side.clone(),
                            order_to_execute.filled_quantity,
                            order_to_execute.avg_fill_price,
                        );

                        if let Err(e) = oms_ui_tx_clone.send(OmsUpdate::OrderStateChange {
                            order_id,
                            new_state: OrderState::Filled,
                            timestamp: chrono::Utc::now(),
                        }) {
                            error!("Failed to send OrderStateChange update to UI: {}", e);
                        }

                        let current_pos = position_manager_clone.read().get_position(&order_to_execute.order.symbol);
                        if let Some(pos) = current_pos {
                             if let Err(e) = oms_ui_tx_clone.send(OmsUpdate::PositionUpdate(pos)) {
                                error!("Failed to send PositionUpdate to UI: {}", e);
                            }
                        }

                    } else {
                        error!("Attempted to simulate fill for non-existent order: {}", order_id);
                    }
                });
            }
            _ = tokio::signal::ctrl_c() => {
                info!("OMS received Ctrl-C, shutting down.");
                break;
            }
        }
    }
    Ok(())
}