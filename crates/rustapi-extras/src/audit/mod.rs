//! Audit logging system for RustAPI
//!
//! This module provides comprehensive audit logging with support for
//! GDPR and SOC2 compliance requirements.
//!
//! # Example
//!
//! ```rust,no_run
//! use rustapi_extras::audit::{AuditEvent, AuditAction, InMemoryAuditStore, AuditStore};
//!
//! // Create an audit store
//! let store = InMemoryAuditStore::new();
//!
//! // Log an audit event
//! let event = AuditEvent::new(AuditAction::Create)
//!     .resource("users", "user-123")
//!     .actor("admin@example.com")
//!     .ip_address("192.168.1.1".parse().unwrap())
//!     .success(true);
//!
//! store.log(event);
//!
//! // Query events
//! let recent = store.query().limit(10).execute();
//! ```

mod event;
mod file_store;
mod memory_store;
mod query;
mod store;

pub use event::{AuditAction, AuditEvent, AuditSeverity, ComplianceInfo};
pub use file_store::FileAuditStore;
pub use memory_store::InMemoryAuditStore;
pub use query::{AuditQuery, AuditQueryBuilder};
pub use store::AuditStore;
