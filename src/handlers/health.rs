use axum::{extract::State, Json};
use serde_json::{json, Value};

use crate::state::AppState;

pub async fn health_check(State(state): State<AppState>) -> Json<Value> {
    let uptime_secs = state.started_at.elapsed().as_secs();
    let store = state.store.read().await;
    let symbols: Vec<&str> = store.symbols();
    Json(json!({
        "status": "ok",
        "uptime_secs": uptime_secs,
        "symbols": symbols,
    }))
}
