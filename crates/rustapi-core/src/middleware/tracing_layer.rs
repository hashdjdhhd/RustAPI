//! Tracing middleware
//!
//! Logs request method, path, status code, and duration for each request.

use super::layer::{BoxedNext, MiddlewareLayer};
use crate::request::Request;
use crate::response::Response;
use std::future::Future;
use std::pin::Pin;
use std::time::Instant;
use tracing::{info, warn, Level};

/// Middleware layer that logs request information
#[derive(Clone)]
pub struct TracingLayer {
    level: Level,
}

impl TracingLayer {
    /// Create a new TracingLayer with default INFO level
    pub fn new() -> Self {
        Self { level: Level::INFO }
    }

    /// Create a TracingLayer with a specific log level
    pub fn with_level(level: Level) -> Self {
        Self { level }
    }
}

impl Default for TracingLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl MiddlewareLayer for TracingLayer {
    fn call(
        &self,
        req: Request,
        next: BoxedNext,
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        let level = self.level;
        let method = req.method().clone();
        let path = req.uri().path().to_string();

        Box::pin(async move {
            let start = Instant::now();

            // Call the next handler
            let response = next(req).await;

            let duration = start.elapsed();
            let status = response.status();

            // Log based on status code
            if status.is_success() {
                match level {
                    Level::TRACE => tracing::trace!(
                        method = %method,
                        path = %path,
                        status = %status.as_u16(),
                        duration_ms = %duration.as_millis(),
                        "Request completed"
                    ),
                    Level::DEBUG => tracing::debug!(
                        method = %method,
                        path = %path,
                        status = %status.as_u16(),
                        duration_ms = %duration.as_millis(),
                        "Request completed"
                    ),
                    Level::INFO => info!(
                        method = %method,
                        path = %path,
                        status = %status.as_u16(),
                        duration_ms = %duration.as_millis(),
                        "Request completed"
                    ),
                    Level::WARN => warn!(
                        method = %method,
                        path = %path,
                        status = %status.as_u16(),
                        duration_ms = %duration.as_millis(),
                        "Request completed"
                    ),
                    Level::ERROR => tracing::error!(
                        method = %method,
                        path = %path,
                        status = %status.as_u16(),
                        duration_ms = %duration.as_millis(),
                        "Request completed"
                    ),
                }
            } else {
                warn!(
                    method = %method,
                    path = %path,
                    status = %status.as_u16(),
                    duration_ms = %duration.as_millis(),
                    "Request failed"
                );
            }

            response
        })
    }

    fn clone_box(&self) -> Box<dyn MiddlewareLayer> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracing_layer_creation() {
        let layer = TracingLayer::new();
        assert_eq!(layer.level, Level::INFO);

        let layer = TracingLayer::with_level(Level::DEBUG);
        assert_eq!(layer.level, Level::DEBUG);
    }
}
