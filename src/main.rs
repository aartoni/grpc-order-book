use std::error::Error;

use futures_util::stream::StreamExt;
use grpc_order_book::{binance, bitstamp, multi_book::MultiBook};
use log::debug;
use tokio_tungstenite::tungstenite::Error::ConnectionClosed;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger
    pretty_env_logger::formatted_builder()
        .filter(None, log::LevelFilter::Debug)
        .init();

    // Define the ticker
    let ticker = String::from("ethbtc");
    debug!("Current ticker: {ticker}");

    // Get readers
    let mut binance_ws = binance::connect(&ticker).await?;
    let mut bitstamp_ws = bitstamp::connect(&ticker).await?;

    // Create an order book
    let mut order_book = MultiBook::new();

    // Read messages from the web socket
    loop {
        tokio::select! {
            message = binance_ws.next() => {
                let message = message
                    .unwrap_or(Err(ConnectionClosed));

                if let Ok(message) = message {
                    let message = binance::parse(message);
                    debug!("Got order book: {message:?}");

                    if let Some(book) = message {
                        order_book.insert("binance", book.into());
                    }
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
