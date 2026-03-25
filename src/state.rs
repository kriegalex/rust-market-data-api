use std::sync::Arc;
use std::time::Instant;

use metrics_exporter_prometheus::PrometheusHandle;
use tokio::sync::RwLock;

use crate::market::MarketStore;

#[derive(Clone)]
pub struct AppState {
    pub store: Arc<RwLock<MarketStore>>,
    pub metrics_handle: PrometheusHandle,
    pub started_at: Instant,
}

impl AppState {
    pub fn new(metrics_handle: PrometheusHandle) -> Self {
        Self {
            store: Arc::new(RwLock::new(MarketStore::new())),
            metrics_handle,
            started_at: Instant::now(),
        }
    }
}
