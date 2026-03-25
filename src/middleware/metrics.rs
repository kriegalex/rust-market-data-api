use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Instant,
};

use axum::http::{Request, Response};
use tower::{Layer, Service};

#[derive(Clone)]
pub struct MetricsLayer;

impl<S> Layer<S> for MetricsLayer {
    type Service = MetricsService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        MetricsService { inner }
    }
}

#[derive(Clone)]
pub struct MetricsService<S> {
    inner: S,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for MetricsService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Send + Clone + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let method = req.method().to_string();
        let path = req.uri().path().to_string();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            let start = Instant::now();
            let response = inner.call(req).await?;
            let latency = start.elapsed().as_secs_f64();
            let status = response.status().as_u16().to_string();

            metrics::counter!(
                "http_requests_total",
                "method" => method.clone(),
                "path" => path.clone(),
                "status" => status
            )
            .increment(1);

            metrics::histogram!(
                "http_request_duration_seconds",
                "method" => method,
                "path" => path
            )
            .record(latency);

            Ok(response)
        })
    }
}
