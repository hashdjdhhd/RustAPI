//! Audit store trait

use super::event::AuditEvent;
use super::query::AuditQueryBuilder;
use std::future::Future;
use std::pin::Pin;

/// Result type for audit operations.
pub type AuditResult<T> = Result<T, AuditError>;

/// Errors that can occur during audit operations.
#[derive(Debug, thiserror::Error)]
pub enum AuditError {
    /// Failed to write audit event.
    #[error("Failed to write audit event: {0}")]
    WriteError(String),

    /// Failed to read audit events.
    #[error("Failed to read audit events: {0}")]
    ReadError(String),

    /// Storage is full.
    #[error("Audit storage is full")]
    StorageFull,

    /// Event not found.
    #[error("Audit event not found: {0}")]
    NotFound(String),

    /// Serialization error.
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// IO error.
    #[error("IO error: {0}")]
    IoError(String),

    /// Configuration error.
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

/// Trait for audit event storage backends.
pub trait AuditStore: Send + Sync {
    /// Log an audit event.
    fn log(&self, event: AuditEvent) -> AuditResult<()>;

    /// Log an audit event asynchronously.
    fn log_async(
        &self,
        event: AuditEvent,
    ) -> Pin<Box<dyn Future<Output = AuditResult<()>> + Send + '_>> {
        Box::pin(async move { self.log(event) })
    }

    /// Get an event by ID.
    fn get(&self, id: &str) -> AuditResult<Option<AuditEvent>>;

    /// Create a query builder.
    fn query(&self) -> AuditQueryBuilder<'_>
    where
        Self: Sized,
    {
        AuditQueryBuilder::new(self)
    }

    /// Execute a query and return matching events.
    fn execute_query(&self, query: &super::query::AuditQuery) -> AuditResult<Vec<AuditEvent>>;

    /// Count events matching the query.
    fn count(&self, query: &super::query::AuditQuery) -> AuditResult<usize>;

    /// Get the total number of stored events.
    fn total_count(&self) -> AuditResult<usize>;

    /// Clear all events (use with caution - for testing).
    fn clear(&self) -> AuditResult<()>;

    /// Flush any buffered events to storage.
    fn flush(&self) -> AuditResult<()>;
}
