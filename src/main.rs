use std::error::Error;

use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio_tungstenite::{connect_async, tungstenite::Message};

const BINANCE_ADDRESS: &str = "wss://stream.binance.com:9443/ws/ethusd@depth10@1000ms";
const BITSTAMP_ADDRESS: &str = "wss://ws.bitstamp.net";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ticker = String::from("ethbtc");
    println!("Current ticker: {ticker}");

    // Get Binance reader
    let (stream, _) = connect_async(BINANCE_ADDRESS).await.unwrap();
    let (_, binance_reader) = stream.split();

    // Get Bitstamp reader and writer
    let (stream, _) = connect_async(BITSTAMP_ADDRESS).await.unwrap();
    let (mut writer, bitstamp_reader) = stream.split();

    // Subscribe to Bitstamp order book stream
    writer.send(Message::Text(json!({
        "event": "bts:subscribe",
        "data": {
            "channel": "order_book_ethusd"
        }
    }).to_string())).await.unwrap();

    loop {
        // ...
    }

    Ok(())
}
