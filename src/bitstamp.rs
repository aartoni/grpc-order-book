use std::error::Error;

use futures_util::SinkExt;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde_json::json;
use tokio_tungstenite::tungstenite::Message;
use crate::{websocket::{self, WsStream}};

const BITSTAMP_ADDRESS: &str = "wss://ws.bitstamp.net";

#[derive(Deserialize)]
pub struct Data {
    #[serde(rename = "data")]
    pub book: Book,
}

#[derive(Deserialize, Debug)]
pub struct Book {
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
}

#[derive(Deserialize, Debug)]
pub struct PriceLevel(pub Decimal, pub Decimal);

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

pub fn parse(msg: Message) -> Option<Book> {
    if let Message::Text(text) = msg {
        return serde_json::from_str(&text)
            .map_or(None, |data: Data| Some(data.book));
    }

    None
}
