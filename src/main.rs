use std::error::Error;

use futures_util::stream::StreamExt;
use grpc_order_book::{binance, bitstamp};
use log::debug;
use tokio_tungstenite::tungstenite::Error::ConnectionClosed;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger
    pretty_env_logger::formatted_builder()
        .filter(None, log::LevelFilter::Debug)
        .init();

    let ticker = String::from("ethbtc");
    debug!("Current ticker: {ticker}");

    // Get readers
    let mut binance_ws = binance::connect(&ticker).await?;
    let mut bitstamp_ws = bitstamp::connect(&ticker).await?;

    loop {
        tokio::select! {
            message = binance_ws.next() => {
                let message = message
                    .unwrap_or(Err(ConnectionClosed));

                if let Ok(message) = message {
                    let message = binance::parse(message);
                    debug!("Got order book: {message:?}");
                }
            },
            message = bitstamp_ws.next() => {
                let message = message
                    .unwrap_or(Err(ConnectionClosed));

                if let Ok(message) = message {
                    let message = bitstamp::parse(message);
                    debug!("Got order book: {message:?}");
                }
            }
        }
    }

    Ok(())
}
