//! SSE event handlers

use rustapi_rs::prelude::*;
use std::sync::Arc;

use crate::models::{Claims, HealthResponse};
use crate::stores::AppState;

/// SSE event stream endpoint - placeholder for now
/// Real SSE implementation would require ResponseModifier for Sse type
#[rustapi_rs::get("/events")]
#[rustapi_rs::tag("Events")]
#[rustapi_rs::summary("SSE Events")]
async fn events(
    State(_state): State<Arc<AppState>>,
    AuthUser(_claims): AuthUser<Claims>,
) -> Json<HealthResponse> {
    // TODO: Implement proper SSE streaming once ResponseModifier is available for Sse
    Json(HealthResponse {
        status: "connected".to_string(),
        version: "SSE endpoint - use EventSource to connect".to_string(),
    })
}
