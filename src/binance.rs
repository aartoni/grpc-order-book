use std::error::Error;

use rust_decimal::Decimal;
use serde::Deserialize;
use tokio_tungstenite::tungstenite::Message;

use crate::websocket::{self, WsStream};

const BINANCE_ADDRESS: &str = "wss://stream.binance.com:9443/ws";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    last_update_id: usize,
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
}

#[derive(Deserialize, Debug)]
pub struct PriceLevel {
    pub price: Decimal,
    pub amount: Decimal,
}

pub async fn connect(symbol: &str) -> Result<WsStream, Box<dyn Error>> {
    let url = format!("{}/{}@depth10@1000ms", BINANCE_ADDRESS, symbol);
    websocket::connect(&url).await
}

pub fn parse(msg: Message) -> Option<Book> {
    if let Message::Text(text) = msg {
        let book = serde_json::from_str(&text).unwrap();
        return Some(book);
    }

    None
}
