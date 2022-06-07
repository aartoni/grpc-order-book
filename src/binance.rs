use std::error::Error;

use crate::websocket::{self, WsStream};

const BINANCE_ADDRESS: &str = "wss://stream.binance.com:9443/ws/ethusd@depth10@1000ms";

pub async fn connect(symbol: &str) -> Result<WsStream, Box<dyn Error>> {
    let url = format!("{}:9443/{}@depth10@1000ms", BINANCE_ADDRESS, symbol);
    websocket::connect(&url).await
}
