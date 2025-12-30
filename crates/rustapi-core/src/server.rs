//! HTTP server implementation

use crate::error::ApiError;
use crate::request::Request;
use crate::response::{IntoResponse, Response};
use crate::router::{RouteMatch, Router};
use bytes::Bytes;
use http::{header, Method, StatusCode};
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{error, info, span, Level};

/// Internal server struct
pub(crate) struct Server {
    router: Arc<Router>,
}

impl Server {
    pub fn new(router: Router) -> Self {
        Self {
            router: Arc::new(router),
        }
    }

    /// Run the server
    pub async fn run(self, addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let addr: SocketAddr = addr.parse()?;
        let listener = TcpListener::bind(addr).await?;

        info!("🚀 RustAPI server running on http://{}", addr);

        loop {
            let (stream, remote_addr) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let router = self.router.clone();

            tokio::spawn(async move {
                let service = service_fn(move |req: hyper::Request<Incoming>| {
                    let router = router.clone();
                    async move {
                        let response = handle_request(router, req, remote_addr).await;
                        Ok::<_, Infallible>(response)
                    }
                });

                if let Err(err) = http1::Builder::new()
                    .serve_connection(io, service)
                    .await
                {
                    error!("Connection error: {}", err);
                }
            });
        }
    }
}

/// Handle a single HTTP request
async fn handle_request(
    router: Arc<Router>,
    req: hyper::Request<Incoming>,
    _remote_addr: SocketAddr,
) -> hyper::Response<Full<Bytes>> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let start = std::time::Instant::now();

    // Match the route
    let response = match router.match_route(&path, &method) {
        RouteMatch::Found { handler, params } => {
            // Convert hyper request to our Request type
            let (parts, body) = req.into_parts();
            
            // Collect body bytes
            let body_bytes = match body.collect().await {
                Ok(collected) => collected.to_bytes(),
                Err(e) => {
                    return ApiError::bad_request(format!("Failed to read body: {}", e))
                        .into_response();
                }
            };

            // Build Request
            let request = Request::new(
                parts,
                body_bytes,
                router.state_ref(),
                params,
            );

            // Call handler
            handler(request).await
        }
        RouteMatch::NotFound => {
            ApiError::not_found(format!("No route found for {} {}", method, path))
                .into_response()
        }
        RouteMatch::MethodNotAllowed { allowed } => {
            let allowed_str: Vec<&str> = allowed.iter().map(|m| m.as_str()).collect();
            let mut response = ApiError::new(
                StatusCode::METHOD_NOT_ALLOWED,
                "method_not_allowed",
                format!("Method {} not allowed for {}", method, path),
            )
            .into_response();
            
            response.headers_mut().insert(
                header::ALLOW,
                allowed_str.join(", ").parse().unwrap(),
            );
            response
        }
    };

    let elapsed = start.elapsed();
    let status = response.status();
    
    // Log request
    if status.is_success() {
        info!(
            method = %method,
            path = %path,
            status = %status.as_u16(),
            duration_ms = %elapsed.as_millis(),
            "Request completed"
        );
    } else {
        error!(
            method = %method,
            path = %path,
            status = %status.as_u16(),
            duration_ms = %elapsed.as_millis(),
            "Request failed"
        );
    }

    response
}
