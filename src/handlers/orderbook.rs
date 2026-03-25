use axum::{
    extract::{Path, State},
    Json,
};

use crate::{error::AppError, market::OrderBook, state::AppState};

pub async fn get_order_book(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
) -> Result<Json<OrderBook>, AppError> {
    let symbol = symbol.to_uppercase();
    let store = state.store.read().await;
    store
        .order_book(&symbol)
        .cloned()
        .map(Json)
        .ok_or_else(|| AppError::SymbolNotFound(symbol))
}
