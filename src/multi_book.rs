use std::collections::HashMap;

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
}
