//! Health check system for monitoring application health
//!
//! This module provides a flexible health check system for monitoring
//! the health and readiness of your application and its dependencies.
//!
//! # Example
//!
//! ```rust,no_run
//! use rustapi_core::health::{HealthCheck, HealthCheckBuilder, HealthStatus};
//!
//! #[tokio::main]
//! async fn main() {
//!     let health = HealthCheckBuilder::new(true)
//!         .add_check("database", || async {
//!             // Check database connection
//!             HealthStatus::healthy()
//!         })
//!         .add_check("redis", || async {
//!             // Check Redis connection
//!             HealthStatus::healthy()
//!         })
//!         .build();
//!
//!     // Use health.execute().await to get results
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Health status of a component
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    /// Component is healthy
    #[serde(rename = "healthy")]
    Healthy,
    /// Component is unhealthy
    #[serde(rename = "unhealthy")]
    Unhealthy { reason: String },
    /// Component is degraded but functional
    #[serde(rename = "degraded")]
    Degraded { reason: String },
}

impl HealthStatus {
    /// Create a healthy status
    pub fn healthy() -> Self {
        Self::Healthy
    }

    /// Create an unhealthy status with a reason
    pub fn unhealthy(reason: impl Into<String>) -> Self {
        Self::Unhealthy {
            reason: reason.into(),
        }
    }

    /// Create a degraded status with a reason
    pub fn degraded(reason: impl Into<String>) -> Self {
        Self::Degraded {
            reason: reason.into(),
        }
    }

    /// Check if the status is healthy
    pub fn is_healthy(&self) -> bool {
        matches!(self, Self::Healthy)
    }

    /// Check if the status is unhealthy
    pub fn is_unhealthy(&self) -> bool {
        matches!(self, Self::Unhealthy { .. })
    }

    /// Check if the status is degraded
    pub fn is_degraded(&self) -> bool {
        matches!(self, Self::Degraded { .. })
    }
}

/// Overall health check result
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckResult {
    /// Overall status
    pub status: HealthStatus,
    /// Individual component checks
    pub checks: HashMap<String, HealthStatus>,
    /// Application version (if provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Timestamp of check (ISO 8601)
    pub timestamp: String,
}

/// Type alias for async health check functions
pub type HealthCheckFn =
    Arc<dyn Fn() -> Pin<Box<dyn Future<Output = HealthStatus> + Send>> + Send + Sync>;

/// Health check configuration
#[derive(Clone)]
pub struct HealthCheck {
    checks: HashMap<String, HealthCheckFn>,
    version: Option<String>,
}

impl HealthCheck {
    /// Execute all health checks
    pub async fn execute(&self) -> HealthCheckResult {
        let mut results = HashMap::new();
        let mut overall_status = HealthStatus::Healthy;

        for (name, check) in &self.checks {
            let status = check().await;

            // Determine overall status
            match &status {
                HealthStatus::Unhealthy { .. } => {
                    overall_status = HealthStatus::unhealthy("one or more checks failed");
                }
                HealthStatus::Degraded { .. } => {
                    if overall_status.is_healthy() {
                        overall_status = HealthStatus::degraded("one or more checks degraded");
                    }
                }
                _ => {}
            }

            results.insert(name.clone(), status);
        }

        // Use UTC timestamp formatted as ISO 8601
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| {
                let secs = d.as_secs();
                let nanos = d.subsec_nanos();
                format!("{}.{:09}Z", secs, nanos)
            })
            .unwrap_or_else(|_| "unknown".to_string());

        HealthCheckResult {
            status: overall_status,
            checks: results,
            version: self.version.clone(),
            timestamp,
        }
    }
}

/// Builder for health check configuration
pub struct HealthCheckBuilder {
    checks: HashMap<String, HealthCheckFn>,
    version: Option<String>,
}

impl HealthCheckBuilder {
    /// Create a new health check builder
    ///
    /// # Arguments
    ///
    /// * `include_default` - Whether to include a default "self" check that always returns healthy
    pub fn new(include_default: bool) -> Self {
        let mut checks = HashMap::new();

        if include_default {
            let check: HealthCheckFn = Arc::new(|| Box::pin(async { HealthStatus::healthy() }));
            checks.insert("self".to_string(), check);
        }

        Self {
            checks,
            version: None,
        }
    }

    /// Add a health check
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustapi_core::health::{HealthCheckBuilder, HealthStatus};
    ///
    /// let health = HealthCheckBuilder::new(false)
    ///     .add_check("database", || async {
    ///         // Simulate database check
    ///         HealthStatus::healthy()
    ///     })
    ///     .build();
    /// ```
    pub fn add_check<F, Fut>(mut self, name: impl Into<String>, check: F) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = HealthStatus> + Send + 'static,
    {
        let check_fn = Arc::new(move || {
            Box::pin(check()) as Pin<Box<dyn Future<Output = HealthStatus> + Send>>
        });
        self.checks.insert(name.into(), check_fn);
        self
    }

    /// Set the application version
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Build the health check
    pub fn build(self) -> HealthCheck {
        HealthCheck {
            checks: self.checks,
            version: self.version,
        }
    }
}

impl Default for HealthCheckBuilder {
    fn default() -> Self {
        Self::new(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn health_check_all_healthy() {
        let health = HealthCheckBuilder::new(false)
            .add_check("db", || async { HealthStatus::healthy() })
            .add_check("cache", || async { HealthStatus::healthy() })
            .version("1.0.0")
            .build();

        let result = health.execute().await;

        assert!(result.status.is_healthy());
        assert_eq!(result.checks.len(), 2);
        assert_eq!(result.version, Some("1.0.0".to_string()));
    }

    #[tokio::test]
    async fn health_check_one_unhealthy() {
        let health = HealthCheckBuilder::new(false)
            .add_check("db", || async { HealthStatus::healthy() })
            .add_check("cache", || async {
                HealthStatus::unhealthy("connection failed")
            })
            .build();

        let result = health.execute().await;

        assert!(result.status.is_unhealthy());
        assert_eq!(result.checks.len(), 2);
    }

    #[tokio::test]
    async fn health_check_one_degraded() {
        let health = HealthCheckBuilder::new(false)
            .add_check("db", || async { HealthStatus::healthy() })
            .add_check("cache", || async { HealthStatus::degraded("high latency") })
            .build();

        let result = health.execute().await;

        assert!(result.status.is_degraded());
        assert_eq!(result.checks.len(), 2);
    }

    #[tokio::test]
    async fn health_check_with_default() {
        let health = HealthCheckBuilder::new(true).build();

        let result = health.execute().await;

        assert!(result.status.is_healthy());
        assert_eq!(result.checks.len(), 1);
        assert!(result.checks.contains_key("self"));
    }
}
