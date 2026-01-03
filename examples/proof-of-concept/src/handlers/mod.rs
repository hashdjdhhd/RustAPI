//! Request handlers for the Bookmark Manager POC

pub mod auth;
pub mod bookmarks;
pub mod categories;
pub mod events;

use rustapi_rs::prelude::*;

use crate::models::HealthResponse;

/// Health check endpoint
#[rustapi_rs::get("/health")]
#[rustapi_rs::tag("System")]
#[rustapi_rs::summary("Health Check")]
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
