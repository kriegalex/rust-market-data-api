mod common;

use rust_decimal::Decimal;
use serde_json::Value;

#[tokio::test]
async fn known_symbol_returns_200_with_book_fields() {
    let server = common::test_server();
    let response = server.get("/orderbook/ETHUSD").await;
    response.assert_status_ok();
    let body: Value = response.json();
    assert_eq!(body["symbol"], "ETHUSD");
    assert!(body["bids"].is_array());
    assert!(body["asks"].is_array());
}

#[tokio::test]
async fn bids_are_sorted_descending() {
    let server = common::test_server();
    let response = server.get("/orderbook/BTCUSD").await;
    response.assert_status_ok();
    let body: Value = response.json();
    let bids = body["bids"].as_array().unwrap();
    assert!(!bids.is_empty());
    for w in bids.windows(2) {
        let a: Decimal = w[0]["price"].as_str().unwrap().parse().unwrap();
        let b: Decimal = w[1]["price"].as_str().unwrap().parse().unwrap();
        assert!(a >= b, "bids must be descending: {} < {}", a, b);
    }
}

#[tokio::test]
async fn asks_are_sorted_ascending() {
    let server = common::test_server();
    let response = server.get("/orderbook/BTCUSD").await;
    response.assert_status_ok();
    let body: Value = response.json();
    let asks = body["asks"].as_array().unwrap();
    assert!(!asks.is_empty());
    for w in asks.windows(2) {
        let a: Decimal = w[0]["price"].as_str().unwrap().parse().unwrap();
        let b: Decimal = w[1]["price"].as_str().unwrap().parse().unwrap();
        assert!(a <= b, "asks must be ascending: {} > {}", a, b);
    }
}

#[tokio::test]
async fn unknown_symbol_returns_404() {
    let server = common::test_server();
    let response = server.get("/orderbook/FAKEUSD").await;
    response.assert_status(axum::http::StatusCode::NOT_FOUND);
    let body: Value = response.json();
    assert!(body["error"].is_string());
}
