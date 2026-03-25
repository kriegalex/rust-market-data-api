use axum_test::TestServer;
use metrics_exporter_prometheus::PrometheusBuilder;
use rust_async_project::{app::create_router, state::AppState};

pub fn test_server() -> TestServer {
    let handle = PrometheusBuilder::new().build_recorder().handle();
    let state = AppState::new(handle);
    TestServer::new(create_router(state)).unwrap()
}
