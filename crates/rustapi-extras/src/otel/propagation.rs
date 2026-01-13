//! W3C Trace Context propagation utilities
//!
//! This module implements trace context propagation according to the
//! W3C Trace Context specification for distributed tracing.

use rustapi_core::Request;
use std::fmt;

/// W3C Trace Context header name for traceparent
pub const TRACEPARENT_HEADER: &str = "traceparent";

/// W3C Trace Context header name for tracestate
pub const TRACESTATE_HEADER: &str = "tracestate";

/// Correlation ID header name
pub const CORRELATION_ID_HEADER: &str = "x-correlation-id";

/// Request ID header name
pub const REQUEST_ID_HEADER: &str = "x-request-id";

/// Trace context information
#[derive(Clone, Debug, Default)]
pub struct TraceContext {
    /// Trace ID (128-bit, hex encoded)
    pub trace_id: String,
    /// Span ID (64-bit, hex encoded)
    pub span_id: String,
    /// Parent span ID (64-bit, hex encoded) - if this is a child span
    pub parent_span_id: Option<String>,
    /// Trace flags (8 bits)
    pub trace_flags: u8,
    /// Trace state (vendor-specific data)
    pub trace_state: Option<String>,
    /// Correlation ID for request tracking
    pub correlation_id: Option<String>,
}

impl TraceContext {
    /// Create a new trace context with generated IDs
    pub fn new() -> Self {
        Self {
            trace_id: Self::generate_trace_id(),
            span_id: Self::generate_span_id(),
            parent_span_id: None,
            trace_flags: 0x01, // Sampled flag
            trace_state: None,
            correlation_id: Some(Self::generate_correlation_id()),
        }
    }

    /// Create a child span context from a parent
    pub fn child(&self) -> Self {
        Self {
            trace_id: self.trace_id.clone(),
            span_id: Self::generate_span_id(),
            parent_span_id: Some(self.span_id.clone()),
            trace_flags: self.trace_flags,
            trace_state: self.trace_state.clone(),
            correlation_id: self.correlation_id.clone(),
        }
    }

    /// Generate a new trace ID (128-bit, 32 hex chars)
    pub fn generate_trace_id() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let random: u64 = rand_simple();
        format!("{:016x}{:016x}", now as u64, random)
    }

    /// Generate a new span ID (64-bit, 16 hex chars)
    pub fn generate_span_id() -> String {
        let random: u64 = rand_simple();
        format!("{:016x}", random)
    }

    /// Generate a correlation ID
    pub fn generate_correlation_id() -> String {
        let random: u64 = rand_simple();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        format!("{:x}-{:x}", timestamp, random)
    }

    /// Check if trace is sampled
    pub fn is_sampled(&self) -> bool {
        self.trace_flags & 0x01 == 0x01
    }

    /// Set sampled flag
    pub fn set_sampled(&mut self, sampled: bool) {
        if sampled {
            self.trace_flags |= 0x01;
        } else {
            self.trace_flags &= !0x01;
        }
    }

    /// Format as W3C traceparent header value
    pub fn to_traceparent(&self) -> String {
        format!(
            "00-{}-{}-{:02x}",
            self.trace_id, self.span_id, self.trace_flags
        )
    }

    /// Parse from W3C traceparent header value
    pub fn from_traceparent(value: &str) -> Option<Self> {
        let parts: Vec<&str> = value.split('-').collect();
        if parts.len() != 4 {
            return None;
        }

        let version = parts[0];
        if version != "00" {
            return None; // Only version 00 is supported
        }

        let trace_id = parts[1];
        let span_id = parts[2];
        let flags = parts[3];

        // Validate lengths
        if trace_id.len() != 32 || span_id.len() != 16 || flags.len() != 2 {
            return None;
        }

        // Parse flags
        let trace_flags = u8::from_str_radix(flags, 16).ok()?;

        Some(Self {
            trace_id: trace_id.to_string(),
            span_id: span_id.to_string(),
            parent_span_id: None,
            trace_flags,
            trace_state: None,
            correlation_id: None,
        })
    }
}

impl fmt::Display for TraceContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_traceparent())
    }
}

/// Extract trace context from incoming request headers
pub fn extract_trace_context(request: &Request) -> TraceContext {
    let headers = request.headers();

    // Try to extract traceparent header
    let mut context = headers
        .get(TRACEPARENT_HEADER)
        .and_then(|v| v.to_str().ok())
        .and_then(TraceContext::from_traceparent)
        .unwrap_or_else(TraceContext::new);

    // Extract tracestate if present
    if let Some(state) = headers.get(TRACESTATE_HEADER).and_then(|v| v.to_str().ok()) {
        context.trace_state = Some(state.to_string());
    }

    // Extract correlation ID from various headers
    context.correlation_id = headers
        .get(CORRELATION_ID_HEADER)
        .or_else(|| headers.get(REQUEST_ID_HEADER))
        .or_else(|| headers.get("x-amzn-trace-id"))
        .and_then(|v| v.to_str().ok())
        .map(String::from)
        .or_else(|| Some(TraceContext::generate_correlation_id()));

    context
}

/// Inject trace context into outgoing request headers
pub fn inject_trace_context(headers: &mut http::HeaderMap, context: &TraceContext) {
    use http::header::HeaderValue;

    // Inject traceparent
    if let Ok(value) = HeaderValue::from_str(&context.to_traceparent()) {
        headers.insert(TRACEPARENT_HEADER, value);
    }

    // Inject tracestate if present
    if let Some(ref state) = context.trace_state {
        if let Ok(value) = HeaderValue::from_str(state) {
            headers.insert(TRACESTATE_HEADER, value);
        }
    }

    // Inject correlation ID
    if let Some(ref correlation_id) = context.correlation_id {
        if let Ok(value) = HeaderValue::from_str(correlation_id) {
            headers.insert(CORRELATION_ID_HEADER, value);
        }
    }
}

/// Propagate trace context to response headers
pub fn propagate_trace_context(response_headers: &mut http::HeaderMap, context: &TraceContext) {
    use http::header::HeaderValue;

    // Include trace ID in response for debugging
    if let Ok(value) = HeaderValue::from_str(&context.trace_id) {
        response_headers.insert("x-trace-id", value);
    }

    // Include correlation ID in response
    if let Some(ref correlation_id) = context.correlation_id {
        if let Ok(value) = HeaderValue::from_str(correlation_id) {
            response_headers.insert(CORRELATION_ID_HEADER, value);
        }
    }
}

/// Simple random number generator (using XorShift)
fn rand_simple() -> u64 {
    use std::cell::Cell;
    use std::time::{SystemTime, UNIX_EPOCH};

    thread_local! {
        static STATE: Cell<u64> = Cell::new(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64
        );
    }

    STATE.with(|state| {
        let mut x = state.get();
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        state.set(x);
        x
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_context_new() {
        let ctx = TraceContext::new();
        assert_eq!(ctx.trace_id.len(), 32);
        assert_eq!(ctx.span_id.len(), 16);
        assert!(ctx.is_sampled());
        assert!(ctx.correlation_id.is_some());
    }

    #[test]
    fn test_trace_context_child() {
        let parent = TraceContext::new();
        let child = parent.child();

        assert_eq!(child.trace_id, parent.trace_id);
        assert_ne!(child.span_id, parent.span_id);
        assert_eq!(child.parent_span_id, Some(parent.span_id));
        assert_eq!(child.correlation_id, parent.correlation_id);
    }

    #[test]
    fn test_traceparent_round_trip() {
        let ctx = TraceContext::new();
        let traceparent = ctx.to_traceparent();
        let parsed = TraceContext::from_traceparent(&traceparent).unwrap();

        assert_eq!(parsed.trace_id, ctx.trace_id);
        assert_eq!(parsed.span_id, ctx.span_id);
        assert_eq!(parsed.trace_flags, ctx.trace_flags);
    }

    #[test]
    fn test_traceparent_parsing() {
        let traceparent = "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01";
        let ctx = TraceContext::from_traceparent(traceparent).unwrap();

        assert_eq!(ctx.trace_id, "0af7651916cd43dd8448eb211c80319c");
        assert_eq!(ctx.span_id, "b7ad6b7169203331");
        assert_eq!(ctx.trace_flags, 0x01);
        assert!(ctx.is_sampled());
    }

    #[test]
    fn test_invalid_traceparent() {
        // Invalid version
        assert!(TraceContext::from_traceparent("01-abc-def-00").is_none());
        // Wrong number of parts
        assert!(TraceContext::from_traceparent("00-abc-def").is_none());
        // Invalid lengths
        assert!(TraceContext::from_traceparent("00-abc-def-00").is_none());
    }

    #[test]
    fn test_sampled_flag() {
        let mut ctx = TraceContext::new();
        assert!(ctx.is_sampled());

        ctx.set_sampled(false);
        assert!(!ctx.is_sampled());

        ctx.set_sampled(true);
        assert!(ctx.is_sampled());
    }
}
