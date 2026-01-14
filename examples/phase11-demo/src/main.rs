//! Phase 11 Features Demo
//!
//! This example demonstrates the new Phase 11 features:
//! - Request Guards (authorization)
//! - Request Timeout
//! - Health Checks
//! - Structured Logging
//! - Circuit Breaker

use rustapi_core::{
    health::{HealthCheckBuilder, HealthStatus},
    RustApi,
};

#[allow(unused_imports)]
use rustapi_extras::{
    CircuitBreakerLayer, LogFormat, LoggingLayer, PermissionGuard, RoleGuard, TimeoutLayer,
};
use std::time::Duration;

#[rustapi_macros::get("/")]
async fn index() -> &'static str {
    "Phase 11 Features Demo"
}

#[rustapi_macros::get("/admin")]
#[cfg(feature = "guard")]
async fn admin_only(guard: RoleGuard) -> Result<&'static str, ApiError> {
    if guard.role != "admin" {
        return Err(ApiError::forbidden("Admin role required".to_string()));
    }
    Ok("Welcome, admin!")
}

#[rustapi_macros::get("/users/edit")]
#[cfg(feature = "guard")]
async fn edit_users(guard: PermissionGuard) -> Result<&'static str, ApiError> {
    if !guard.permissions.contains(&"users.edit".to_string()) {
        return Err(ApiError::forbidden("Missing permission: users.edit".to_string()));
    }
    Ok("Editing users")
}

#[rustapi_macros::get("/slow")]
async fn slow_endpoint() -> &'static str {
    // This would timeout with a 30s timeout
    tokio::time::sleep(Duration::from_secs(35)).await;
    "This should timeout"
}

#[derive(serde::Serialize, rustapi_rs::prelude::Schema)]
struct HealthResponse {
    status: String,
    #[schema(value_type = std::collections::HashMap<String, String>)]
    checks: serde_json::Value,
    version: Option<String>,
    timestamp: String,
}

#[rustapi_macros::get("/health")]
async fn health_endpoint() -> rustapi_core::Json<HealthResponse> {
    // Create health check
    let health = HealthCheckBuilder::new(true)
        .add_check("database", || async {
            // Simulate database check
            tokio::time::sleep(Duration::from_millis(10)).await;
            HealthStatus::healthy()
        })
        .add_check("cache", || async {
            // Simulate cache check
            tokio::time::sleep(Duration::from_millis(5)).await;
            HealthStatus::healthy()
        })
        .version("1.0.0")
        .build();

    let result = health.execute().await;
    
    // Map status enum to string manually or use debug/string representation
    let status_str = match result.status {
        HealthStatus::Healthy => "healthy".to_string(),
        HealthStatus::Unhealthy { .. } => "unhealthy".to_string(),
        HealthStatus::Degraded { .. } => "degraded".to_string(),
    };

    rustapi_core::Json(HealthResponse {
        status: status_str,
        checks: serde_json::to_value(result.checks).unwrap_or(serde_json::Value::Null),
        version: result.version,
        timestamp: result.timestamp,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let mut app = RustApi::new();

    // Add timeout middleware (30 seconds)
    #[cfg(feature = "timeout")]
    {
        app = app.layer(TimeoutLayer::from_secs(30));
    }

    // Add logging middleware
    #[cfg(feature = "logging")]
    {
        app = app.layer(
            LoggingLayer::new()
                .format(LogFormat::Detailed)
                .log_request_headers(true)
                .log_response_headers(true)
                .skip_path("/health")
                .skip_path("/metrics"),
        );
    }

    // Add circuit breaker middleware
    #[cfg(feature = "circuit-breaker")]
    {
        app = app.layer(
            CircuitBreakerLayer::new()
                .failure_threshold(5)
                .timeout(Duration::from_secs(60))
                .success_threshold(2),
        );
    }

    // Mount routes
    // Routes are automatically mounted via rustapi_macros::get

    #[cfg(not(feature = "guard"))]
    {
        // If guards are disabled, we might want to mount these anyway but the handlers have guards?
        // Actually, the handlers themselves have #[cfg(feature = "guard")].
        // So if the feature is off, the handlers don't exist, so we can't mount them anyway.
        // Auto-mounting handles this naturally.
    }

    println!("🚀 Phase 11 Demo running on http://localhost:3000");
    println!();
    println!("Available endpoints:");
    println!("  GET /              - Index page");
    println!("  GET /health        - Health check (with custom checks)");
    println!("  GET /slow          - Slow endpoint (will timeout after 30s)");

    #[cfg(feature = "guard")]
    {
        println!("  GET /admin         - Admin only (requires admin role)");
        println!("  GET /users/edit    - Edit users (requires users.edit permission)");
    }

    println!();
    println!("Features enabled:");
    
    #[cfg(feature = "timeout")]
    println!("  ✓ Timeout middleware (30s)");
    
    #[cfg(feature = "logging")]
    println!("  ✓ Structured logging (detailed format)");
    
    #[cfg(feature = "circuit-breaker")]
    println!("  ✓ Circuit breaker (5 failures, 60s timeout)");
    
    #[cfg(feature = "guard")]
    println!("  ✓ Request guards (role & permission based)");

    app.run("127.0.0.1:3000").await
}
