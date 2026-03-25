use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

use crate::{handlers, middleware::MetricsLayer, state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(handlers::health::health_check))
        .route("/ticker/{symbol}", get(handlers::ticker::get_ticker))
        .route("/orderbook/{symbol}", get(handlers::orderbook::get_order_book))
        .route("/metrics", get(handlers::metrics::prometheus_metrics))
        .layer(MetricsLayer)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
