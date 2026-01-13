//! Query builder for audit events

use super::event::{AuditAction, AuditEvent, AuditSeverity};
use super::store::{AuditResult, AuditStore};

/// Query parameters for filtering audit events.
#[derive(Debug, Clone, Default)]
pub struct AuditQuery {
    /// Filter by actor ID.
    pub actor_id: Option<String>,
    /// Filter by action type.
    pub action: Option<AuditAction>,
    /// Filter by resource type.
    pub resource_type: Option<String>,
    /// Filter by resource ID.
    pub resource_id: Option<String>,
    /// Filter by success/failure.
    pub success: Option<bool>,
    /// Filter by minimum severity.
    pub min_severity: Option<AuditSeverity>,
    /// Filter by start timestamp (inclusive).
    pub from_timestamp: Option<u64>,
    /// Filter by end timestamp (inclusive).
    pub to_timestamp: Option<u64>,
    /// Filter by request ID.
    pub request_id: Option<String>,
    /// Filter by session ID.
    pub session_id: Option<String>,
    /// Filter events involving personal data.
    pub involves_personal_data: Option<bool>,
    /// Filter by IP address.
    pub ip_address: Option<String>,
    /// Maximum number of results.
    pub limit: Option<usize>,
    /// Offset for pagination.
    pub offset: Option<usize>,
    /// Sort order (true = newest first).
    pub newest_first: bool,
}

impl AuditQuery {
    /// Create a new empty query.
    pub fn new() -> Self {
        Self {
            newest_first: true,
            ..Default::default()
        }
    }

    /// Check if an event matches this query.
    pub fn matches(&self, event: &AuditEvent) -> bool {
        // Actor filter
        if let Some(ref actor) = self.actor_id {
            if event.actor_id.as_ref() != Some(actor) {
                return false;
            }
        }

        // Action filter
        if let Some(ref action) = self.action {
            if &event.action != action {
                return false;
            }
        }

        // Resource type filter
        if let Some(ref rt) = self.resource_type {
            if event.resource_type.as_ref() != Some(rt) {
                return false;
            }
        }

        // Resource ID filter
        if let Some(ref rid) = self.resource_id {
            if event.resource_id.as_ref() != Some(rid) {
                return false;
            }
        }

        // Success filter
        if let Some(success) = self.success {
            if event.success != success {
                return false;
            }
        }

        // Severity filter
        if let Some(min_sev) = self.min_severity {
            if event.severity < min_sev {
                return false;
            }
        }

        // Timestamp filters
        if let Some(from) = self.from_timestamp {
            if event.timestamp < from {
                return false;
            }
        }

        if let Some(to) = self.to_timestamp {
            if event.timestamp > to {
                return false;
            }
        }

        // Request ID filter
        if let Some(ref req_id) = self.request_id {
            if event.request_id.as_ref() != Some(req_id) {
                return false;
            }
        }

        // Session ID filter
        if let Some(ref sess_id) = self.session_id {
            if event.session_id.as_ref() != Some(sess_id) {
                return false;
            }
        }

        // Personal data filter
        if let Some(personal) = self.involves_personal_data {
            if event.compliance.involves_personal_data != personal {
                return false;
            }
        }

        // IP address filter
        if let Some(ref ip) = self.ip_address {
            if event.ip_address.as_ref() != Some(ip) {
                return false;
            }
        }

        true
    }
}

/// Builder for constructing audit queries.
pub struct AuditQueryBuilder<'a> {
    store: &'a dyn AuditStore,
    query: AuditQuery,
}

impl<'a> AuditQueryBuilder<'a> {
    /// Create a new query builder.
    pub fn new(store: &'a dyn AuditStore) -> Self {
        Self {
            store,
            query: AuditQuery::new(),
        }
    }

    /// Filter by actor ID.
    pub fn actor(mut self, actor_id: impl Into<String>) -> Self {
        self.query.actor_id = Some(actor_id.into());
        self
    }

    /// Filter by action.
    pub fn action(mut self, action: AuditAction) -> Self {
        self.query.action = Some(action);
        self
    }

    /// Filter by resource type.
    pub fn resource_type(mut self, resource_type: impl Into<String>) -> Self {
        self.query.resource_type = Some(resource_type.into());
        self
    }

    /// Filter by resource ID.
    pub fn resource_id(mut self, resource_id: impl Into<String>) -> Self {
        self.query.resource_id = Some(resource_id.into());
        self
    }

    /// Filter by resource (type and ID).
    pub fn resource(
        mut self,
        resource_type: impl Into<String>,
        resource_id: impl Into<String>,
    ) -> Self {
        self.query.resource_type = Some(resource_type.into());
        self.query.resource_id = Some(resource_id.into());
        self
    }

    /// Filter by success.
    pub fn success(mut self, success: bool) -> Self {
        self.query.success = Some(success);
        self
    }

    /// Filter by failures only.
    pub fn failures_only(self) -> Self {
        self.success(false)
    }

    /// Filter by minimum severity.
    pub fn min_severity(mut self, severity: AuditSeverity) -> Self {
        self.query.min_severity = Some(severity);
        self
    }

    /// Filter events from a timestamp.
    pub fn from_timestamp(mut self, timestamp: u64) -> Self {
        self.query.from_timestamp = Some(timestamp);
        self
    }

    /// Filter events until a timestamp.
    pub fn to_timestamp(mut self, timestamp: u64) -> Self {
        self.query.to_timestamp = Some(timestamp);
        self
    }

    /// Filter by time range.
    pub fn time_range(mut self, from: u64, to: u64) -> Self {
        self.query.from_timestamp = Some(from);
        self.query.to_timestamp = Some(to);
        self
    }

    /// Filter by request ID.
    pub fn request_id(mut self, request_id: impl Into<String>) -> Self {
        self.query.request_id = Some(request_id.into());
        self
    }

    /// Filter by session ID.
    pub fn session_id(mut self, session_id: impl Into<String>) -> Self {
        self.query.session_id = Some(session_id.into());
        self
    }

    /// Filter events involving personal data.
    pub fn personal_data(mut self, involves: bool) -> Self {
        self.query.involves_personal_data = Some(involves);
        self
    }

    /// Filter by IP address.
    pub fn ip_address(mut self, ip: impl Into<String>) -> Self {
        self.query.ip_address = Some(ip.into());
        self
    }

    /// Limit results.
    pub fn limit(mut self, limit: usize) -> Self {
        self.query.limit = Some(limit);
        self
    }

    /// Set offset for pagination.
    pub fn offset(mut self, offset: usize) -> Self {
        self.query.offset = Some(offset);
        self
    }

    /// Sort newest first (default).
    pub fn newest_first(mut self) -> Self {
        self.query.newest_first = true;
        self
    }

    /// Sort oldest first.
    pub fn oldest_first(mut self) -> Self {
        self.query.newest_first = false;
        self
    }

    /// Execute the query.
    pub fn execute(self) -> AuditResult<Vec<AuditEvent>> {
        self.store.execute_query(&self.query)
    }

    /// Count matching events.
    pub fn count(self) -> AuditResult<usize> {
        self.store.count(&self.query)
    }

    /// Get the built query.
    pub fn build(self) -> AuditQuery {
        self.query
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_matches() {
        let event = AuditEvent::new(AuditAction::Create)
            .resource("users", "user-123")
            .actor("admin")
            .success(true);

        // Matching query
        let query = AuditQuery {
            action: Some(AuditAction::Create),
            resource_type: Some("users".to_string()),
            ..Default::default()
        };
        assert!(query.matches(&event));

        // Non-matching query
        let query = AuditQuery {
            action: Some(AuditAction::Delete),
            ..Default::default()
        };
        assert!(!query.matches(&event));
    }

    #[test]
    fn test_query_severity_filter() {
        let info_event = AuditEvent::new(AuditAction::Read).severity(AuditSeverity::Info);

        let warning_event =
            AuditEvent::new(AuditAction::LoginFailed).severity(AuditSeverity::Warning);

        let query = AuditQuery {
            min_severity: Some(AuditSeverity::Warning),
            ..Default::default()
        };

        assert!(!query.matches(&info_event));
        assert!(query.matches(&warning_event));
    }
}
