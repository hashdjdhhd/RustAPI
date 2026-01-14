//! Structured Logging Layer for RustAPI
//!
//! This module provides advanced structured logging with support for:
//! - Multiple log formats (JSON, Datadog, Splunk, Logfmt)
//! - Correlation ID injection
//! - Configurable field inclusion
//! - Log level filtering
//!
//! # Example
//!
//! ```rust,no_run
//! use rustapi_core::RustApi;
//! use rustapi_extras::structured_logging::{StructuredLoggingConfig, StructuredLoggingLayer, LogOutputFormat};
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = StructuredLoggingConfig::builder()
//!         .format(LogOutputFormat::Json)
//!         .include_request_headers(true)
//!         .correlation_id_header("x-request-id")
//!         .build();
//!
//!     let app = RustApi::new()
//!         .layer(StructuredLoggingLayer::new(config))
//!         .run("0.0.0.0:3000")
//!         .await
//!         .unwrap();
//! }
//! ```

mod config;
mod formats;
mod layer;

pub use config::{LogOutputFormat, StructuredLoggingConfig, StructuredLoggingConfigBuilder};
pub use formats::{
    DatadogFormatter, JsonFormatter, LogFormatter, LogfmtFormatter, SplunkFormatter,
};
pub use layer::StructuredLoggingLayer;
