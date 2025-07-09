use tungstenite::Message;
use url::Url;
use futures_util::{StreamExt, SinkExt};
use tokio::sync::mpsc;
use tracing::{info, error, debug};
use serde_json::Value;
use anyhow::{Result, anyhow};
use tokio_tungstenite;

use crate::data_model::MarketDataUpdate;

pub async fn connect_and_stream(
    url_str: String,
    md_tx: mpsc::UnboundedSender<MarketDataUpdate>,
) -> Result<()> {
    let url = Url::parse(&url_str)?;
    info!("Connecting to WebSocket: {}", url_str);

    let (ws_stream, _) = tokio_tungstenite::connect_async(url).await
        .map_err(|e| anyhow!("Failed to connect to WebSocket: {}", e))?;

    info!("WebSocket connected.");

    let (mut write, mut read) = ws_stream.split();

    let (tx_ws_out, mut rx_ws_out) = mpsc::unbounded_channel::<Message>();

    tokio::spawn(async move {
        while let Some(message) = rx_ws_out.recv().await {
            if let Err(e) = write.send(message).await {
                error!("Failed to send message to WebSocket: {}", e);
                break;
            }
        }
    });

    let tx_ws_out_ping = tx_ws_out.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
            if let Err(e) = tx_ws_out_ping.send(Message::Ping(vec![])) {
                error!("Failed to send WebSocket ping via channel: {}", e);
                break; 
            }
            debug!("Sent WebSocket ping.");
        }
    });


    while let Some(msg_res) = read.next().await {
        match msg_res {
            Ok(msg) => {
                match msg {
                    Message::Text(text) => {
                        match parse_binance_agg_trade(&text) {
                            Ok(update) => {
                                if let Err(e) = md_tx.send(update) {
                                    error!("Failed to send market data update to UI: {}", e);
                                    break;
                                }
                            }
                            Err(e) => {
                                error!("Failed to parse market data JSON: {} - {}", e, text);
                            }
                        }
                    },
                    Message::Ping(p) => {
                        debug!("Received Ping: {:?}", p);
                        if let Err(e) = tx_ws_out.send(Message::Pong(p)) {
                            error!("Failed to send Pong via channel: {}", e);
                            break; 
                        }
                    },
                    Message::Pong(p) => {
                        debug!("Received Pong: {:?}", p);
                    },
                    Message::Binary(b) => {
                        debug!("Received Binary message ({} bytes)", b.len());
                    },
                    Message::Close(cf) => {
                        info!("WebSocket closed: {:?}", cf);
                        break;
                    },
                    _ => {}
                }
            },
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
        }
    }

    error!("WebSocket stream ended.");
    Ok(())
}

fn parse_binance_agg_trade(json_str: &str) -> Result<MarketDataUpdate> {
    let v: Value = serde_json::from_str(json_str)?;

    let event_type = v["e"].as_str().unwrap_or_default();

    if event_type == "aggTrade" {
        let symbol = v["s"].as_str().ok_or_else(|| anyhow!("Missing symbol"))?.to_string();
        let price = v["p"].as_str().unwrap_or("0.0").parse::<f64>()?;
        let quantity = v["q"].as_str().unwrap_or("0.0").parse::<f64>()?;
        let timestamp_ms = v["E"].as_u64().ok_or_else(|| anyhow!("Missing timestamp"))?;
        let timestamp = chrono::DateTime::from_timestamp_millis(timestamp_ms as i64)
                                .ok_or_else(|| anyhow!("Invalid timestamp"))?;

        Ok(MarketDataUpdate {
            symbol,
            timestamp,
            bid_price: None,
            bid_quantity: None,
            ask_price: None,
            ask_quantity: None,
            last_price: Some(price),
            last_quantity: Some(quantity),
        })
    } else {
        Err(anyhow!("Unsupported event type: {}", event_type))
    }
}