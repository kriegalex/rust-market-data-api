mod common;

use serde_json::Value;

#[tokio::test]
async fn known_symbol_returns_200_with_ticker_fields() {
    let server = common::test_server();
    let response = server.get("/ticker/BTCUSD").await;
    response.assert_status_ok();
    let body: Value = response.json();
    assert_eq!(body["symbol"], "BTCUSD");
    assert!(body["bid"].is_string());
    assert!(body["ask"].is_string());
    assert!(body["last"].is_string());
    assert!(body["volume_24h"].is_string());
}

#[tokio::test]
async fn lowercase_symbol_is_normalized() {
    let server = common::test_server();
    let response = server.get("/ticker/btcusd").await;
    response.assert_status_ok();
    let body: Value = response.json();
    assert_eq!(body["symbol"], "BTCUSD");
}

#[tokio::test]
async fn unknown_symbol_returns_404() {
    let server = common::test_server();
    let response = server.get("/ticker/FAKEUSD").await;
    response.assert_status(axum::http::StatusCode::NOT_FOUND);
    let body: Value = response.json();
    assert!(body["error"].is_string());
}
