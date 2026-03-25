use std::collections::HashMap;

use rust_decimal_macros::dec;

use super::model::{Level, OrderBook, Ticker};

pub struct MarketStore {
    tickers: HashMap<String, Ticker>,
    books: HashMap<String, OrderBook>,
}

impl MarketStore {
    pub fn new() -> Self {
        let mut tickers = HashMap::new();
        let mut books = HashMap::new();

        tickers.insert(
            "BTCUSD".into(),
            Ticker {
                symbol: "BTCUSD".into(),
                bid: dec!(67_450.00),
                ask: dec!(67_451.50),
                last: dec!(67_450.75),
                volume_24h: dec!(1_234.567),
            },
        );
        tickers.insert(
            "ETHUSD".into(),
            Ticker {
                symbol: "ETHUSD".into(),
                bid: dec!(3_510.25),
                ask: dec!(3_510.75),
                last: dec!(3_510.50),
                volume_24h: dec!(45_678.9),
            },
        );
        tickers.insert(
            "SOLUSD".into(),
            Ticker {
                symbol: "SOLUSD".into(),
                bid: dec!(178.40),
                ask: dec!(178.45),
                last: dec!(178.42),
                volume_24h: dec!(987_654.3),
            },
        );

        books.insert(
            "BTCUSD".into(),
            OrderBook {
                symbol: "BTCUSD".into(),
                bids: vec![
                    Level { price: dec!(67_450.00), quantity: dec!(0.5) },
                    Level { price: dec!(67_449.00), quantity: dec!(1.2) },
                    Level { price: dec!(67_445.00), quantity: dec!(2.0) },
                ],
                asks: vec![
                    Level { price: dec!(67_451.50), quantity: dec!(0.3) },
                    Level { price: dec!(67_452.00), quantity: dec!(0.8) },
                    Level { price: dec!(67_455.00), quantity: dec!(1.5) },
                ],
            },
        );
        books.insert(
            "ETHUSD".into(),
            OrderBook {
                symbol: "ETHUSD".into(),
                bids: vec![
                    Level { price: dec!(3_510.25), quantity: dec!(5.0) },
                    Level { price: dec!(3_509.00), quantity: dec!(10.5) },
                    Level { price: dec!(3_505.00), quantity: dec!(20.0) },
                ],
                asks: vec![
                    Level { price: dec!(3_510.75), quantity: dec!(4.0) },
                    Level { price: dec!(3_511.50), quantity: dec!(8.0) },
                    Level { price: dec!(3_515.00), quantity: dec!(15.0) },
                ],
            },
        );
        books.insert(
            "SOLUSD".into(),
            OrderBook {
                symbol: "SOLUSD".into(),
                bids: vec![
                    Level { price: dec!(178.40), quantity: dec!(100.0) },
                    Level { price: dec!(178.30), quantity: dec!(250.0) },
                    Level { price: dec!(178.00), quantity: dec!(500.0) },
                ],
                asks: vec![
                    Level { price: dec!(178.45), quantity: dec!(80.0) },
                    Level { price: dec!(178.50), quantity: dec!(200.0) },
                    Level { price: dec!(179.00), quantity: dec!(400.0) },
                ],
            },
        );

        Self { tickers, books }
    }

    pub fn ticker(&self, symbol: &str) -> Option<&Ticker> {
        self.tickers.get(symbol)
    }

    pub fn order_book(&self, symbol: &str) -> Option<&OrderBook> {
        self.books.get(symbol)
    }

    pub fn symbols(&self) -> Vec<&str> {
        let mut syms: Vec<&str> = self.tickers.keys().map(|s| s.as_str()).collect();
        syms.sort();
        syms
    }
}

impl Default for MarketStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_symbols_accessible() {
        let store = MarketStore::new();
        assert!(store.ticker("BTCUSD").is_some());
        assert!(store.ticker("ETHUSD").is_some());
        assert!(store.ticker("SOLUSD").is_some());
        assert!(store.order_book("BTCUSD").is_some());
        assert!(store.order_book("ETHUSD").is_some());
        assert!(store.order_book("SOLUSD").is_some());
    }

    #[test]
    fn unknown_symbol_returns_none() {
        let store = MarketStore::new();
        assert!(store.ticker("FAKEUSD").is_none());
        assert!(store.order_book("FAKEUSD").is_none());
    }

    #[test]
    fn symbols_returns_all_known() {
        let store = MarketStore::new();
        let syms = store.symbols();
        assert!(syms.contains(&"BTCUSD"));
        assert!(syms.contains(&"ETHUSD"));
        assert!(syms.contains(&"SOLUSD"));
        assert_eq!(syms.len(), 3);
    }

    #[test]
    fn bids_sorted_descending() {
        let store = MarketStore::new();
        let book = store.order_book("BTCUSD").unwrap();
        for w in book.bids.windows(2) {
            assert!(w[0].price >= w[1].price, "bids must be descending");
        }
    }

    #[test]
    fn asks_sorted_ascending() {
        let store = MarketStore::new();
        let book = store.order_book("BTCUSD").unwrap();
        for w in book.asks.windows(2) {
            assert!(w[0].price <= w[1].price, "asks must be ascending");
        }
    }
}
