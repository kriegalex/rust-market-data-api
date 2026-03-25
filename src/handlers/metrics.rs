use axum::{
    extract::State,
    http::header,
    response::{IntoResponse, Response},
};

use crate::state::AppState;

pub async fn prometheus_metrics(State(state): State<AppState>) -> Response {
    let body = state.metrics_handle.render();
    ([(header::CONTENT_TYPE, "text/plain; version=0.0.4")], body).into_response()
}
