mod common;

use serde_json::Value;

#[tokio::test]
async fn health_returns_200_with_ok_status() {
    let server = common::test_server();
    let response = server.get("/health").await;
    response.assert_status_ok();
    let body: Value = response.json();
    assert_eq!(body["status"], "ok");
}

#[tokio::test]
async fn health_includes_uptime_and_symbols() {
    let server = common::test_server();
    let response = server.get("/health").await;
    response.assert_status_ok();
    let body: Value = response.json();
    assert!(body["uptime_secs"].is_number());
    assert!(body["symbols"].is_array());
    assert!(!body["symbols"].as_array().unwrap().is_empty());
}
