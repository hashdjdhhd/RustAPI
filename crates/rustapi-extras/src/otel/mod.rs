//! OpenTelemetry Integration for RustAPI
//!
//! This module provides OpenTelemetry integration with support for:
//! - Distributed tracing with OTLP exporter
//! - Metrics collection
//! - Trace context propagation (W3C Trace Context)
//! - Automatic span creation for HTTP requests
//!
//! # Example
//!
//! ```rust,no_run
//! use rustapi_core::RustApi;
//! use rustapi_extras::otel::{OtelConfig, OtelLayer};
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = OtelConfig::builder()
//!         .service_name("my-api")
//!         .endpoint("http://localhost:4317")
//!         .build();
//!
//!     let app = RustApi::new()
//!         .layer(OtelLayer::new(config))
//!         .run("0.0.0.0:3000")
//!         .await
//!         .unwrap();
//! }
//! ```

mod config;
mod layer;
mod propagation;

pub use config::{OtelConfig, OtelConfigBuilder, OtelExporter, TraceSampler};
pub use layer::OtelLayer;
pub use propagation::{
    extract_trace_context, inject_trace_context, propagate_trace_context, TraceContext,
};
