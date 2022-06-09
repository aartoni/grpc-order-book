use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{bitstamp, binance};

pub struct OrderBook {
    bids: Vec<PriceLevel>,
    asks: Vec<PriceLevel>,
}

impl OrderBook {
    #[must_use]
    pub fn new() -> Self {
        Self {
            bids: Vec::with_capacity(10),
            asks: Vec::with_capacity(10)
        }
    }

    pub fn with_prices(bids: Vec<PriceLevel>, asks: Vec<PriceLevel>) -> Self {
        Self { bids, asks }
    }
}

impl From<binance::Book> for OrderBook {
    fn from(other: binance::Book) -> Self {
        Self {
            bids: other.bids.into_iter()
                .map(std::convert::Into::into)
                .collect(),
            asks: other.asks.into_iter()
                .map(std::convert::Into::into)
                .collect(),
        }
    }
}

impl From<bitstamp::Book> for OrderBook {
    fn from(other: bitstamp::Book) -> Self {
        Self {
            bids: other.bids.into_iter()
                .take(10)
                .map(std::convert::Into::into)
                .collect(),
            asks: other.asks.into_iter()
                .take(10)
                .map(std::convert::Into::into)
                .collect(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct PriceLevel {
    exchange: String,
    price: Decimal,
    amount: Decimal,
}

impl From<binance::PriceLevel> for PriceLevel {
    fn from(price_level: binance::PriceLevel) -> Self {
        Self {
            exchange: String::from("binance"),
            price: price_level.price,
            amount: price_level.amount,
        }
    }
}

impl From<bitstamp::PriceLevel> for PriceLevel {
    fn from(price_level: bitstamp::PriceLevel) -> Self {
        Self {
            exchange: String::from("bitstamp"),
            price: price_level.0,
            amount: price_level.1,
        }
    }
}
