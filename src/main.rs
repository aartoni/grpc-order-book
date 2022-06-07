use std::error::Error;

use grpc_order_book::{binance, bitstamp};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ticker = String::from("ethbtc");
    println!("Current ticker: {ticker}");

    // Get readers
    let binance_reader = binance::connect(&ticker);
    let bitstamp_reader = bitstamp::connect(&ticker);

    loop {
        // ...
    }

    Ok(())
}
