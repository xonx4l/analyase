use crate::data_model::{MarketDataUpdate, Order, Side, OrderType, TimeInForce};
use crate::config::StrategyConfig;
use tracing::{info, debug};

pub struct MeanReversionStrategy {
    config: StrategyConfig,
    
}

impl MeanReversionStrategy {
    pub fn new(config: StrategyConfig) -> Self {
        Self {
            config,
           
        }
    }

    pub fn process_market_data(&mut self, update: &MarketDataUpdate) -> Option<Order> {
    

        if let Some(last_price) = update.last_price {
            debug!("Mean Reversion: Processing price {} for {}", last_price, update.symbol);

        
            if last_price > 25000.0 && self.config.name == "MeanReversion" && self.config.enabled {
                info!("Mean Reversion: Buy signal generated for {}", update.symbol);
                return Some(Order::new(
                    update.symbol.clone(),
                    Side::Sell, 
                    OrderType::Limit,
                    0.0001, 
                    Some(last_price), 
                    TimeInForce::GTC,
                ));
            } else if last_price < 19000.0 && self.config.name == "MeanReversion" && self.config.enabled {
                info!("Mean Reversion: Sell signal generated for {}", update.symbol);
                return Some(Order::new(
                    update.symbol.clone(),
                    Side::Buy, 
                    OrderType::Limit,
                    0.0001,
                    Some(last_price),
                    TimeInForce::GTC,
                ));
            }
        }
        None 
    }

    pub fn update_config(&mut self, new_config: StrategyConfig) {
        self.config = new_config;
        info!("Mean Reversion strategy config updated: {:?}", self.config);
    }
}