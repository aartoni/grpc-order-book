use std::error::Error;

use futures_util::SinkExt;
use serde_json::json;
use tokio_tungstenite::tungstenite::Message;
use crate::websocket::{self, WsStream};

const BITSTAMP_ADDRESS: &str = "wss://ws.bitstamp.net";

pub async fn connect(symbol: &str) -> Result<WsStream, Box<dyn Error>> {
    let mut ws_stream = websocket::connect(BITSTAMP_ADDRESS).await?;
    subscribe(&mut ws_stream, symbol).await?;
    Ok(ws_stream)
}

pub async fn subscribe(ws_stream: &mut WsStream, symbol: &str) -> Result<(), Box<dyn Error>> {
    ws_stream.send(Message::Text(json!({
        "event": "bts:subscribe",
        "data": {
            "channel": format!("order_book_{symbol}")
        }
    }).to_string())).await?;

    Ok(())
}
