use std::collections::HashMap;

use log::debug;

use crate::order_book::OrderBook;

pub struct MultiBook {
    books: HashMap<String, OrderBook>,
}

impl MultiBook {
    #[must_use]
    pub fn new() -> Self {
        Self { books: HashMap::with_capacity(2) }
    }

    pub fn insert(&mut self, symbol: &str, book: OrderBook) -> Option<OrderBook> {
        self.books.insert(symbol.into(), book)
    }

    pub fn merge_book(&self) -> OrderBook {
        debug!("Merging books");

        // Chain iterators over each order book

        // Merge bids and asks for each one

        // Sort the arrays

        // Return the merged order book
        let bids = vec![];
        let asks = vec![];
        OrderBook::with_prices(bids, asks)
    }
}
