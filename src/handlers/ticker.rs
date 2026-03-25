use axum::{
    extract::{Path, State},
    Json,
};

use crate::{error::AppError, market::Ticker, state::AppState};

pub async fn get_ticker(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
) -> Result<Json<Ticker>, AppError> {
    let symbol = symbol.to_uppercase();
    let store = state.store.read().await;
    store
        .ticker(&symbol)
        .cloned()
        .map(Json)
        .ok_or_else(|| AppError::SymbolNotFound(symbol))
}
