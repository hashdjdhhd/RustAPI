//! Structured logging configuration types

use std::collections::HashSet;

/// Log output format
#[derive(Clone, Debug, Default, PartialEq)]
pub enum LogOutputFormat {
    /// JSON format (default)
    #[default]
    Json,
    /// Datadog APM format
    Datadog,
    /// Splunk HEC format
    Splunk,
    /// Logfmt format (key=value pairs)
    Logfmt,
    /// Pretty format (human-readable, for development)
    Pretty,
}

/// Structured logging configuration
#[derive(Clone, Debug)]
pub struct StructuredLoggingConfig {
    /// Output format for logs
    pub format: LogOutputFormat,
    /// Whether to include request headers in logs
    pub include_request_headers: bool,
    /// Whether to include response headers in logs
    pub include_response_headers: bool,
    /// Whether to include request body (for debugging)
    pub include_request_body: bool,
    /// Whether to include response body (for debugging)
    pub include_response_body: bool,
    /// Maximum body size to log (in bytes)
    pub max_body_size: usize,
    /// Header name in which the correlation ID is passed
    pub correlation_id_header: String,
    /// Whether to generate correlation ID if not present
    pub generate_correlation_id: bool,
    /// Headers to redact from logs (for security)
    pub redact_headers: HashSet<String>,
    /// Whether to include timing information
    pub include_timing: bool,
    /// Service name for logs
    pub service_name: String,
    /// Service version
    pub service_version: Option<String>,
    /// Environment name (production, staging, etc.)
    pub environment: Option<String>,
    /// Paths to exclude from logging
    pub exclude_paths: Vec<String>,
    /// Additional static fields to include in all logs
    pub static_fields: Vec<(String, String)>,
    /// Whether to log at request start
    pub log_request_start: bool,
    /// Whether to log at request end
    pub log_request_end: bool,
    /// Whether to include caller info (file, line)
    pub include_caller_info: bool,
}

impl Default for StructuredLoggingConfig {
    fn default() -> Self {
        let mut redact_headers = HashSet::new();
        redact_headers.insert("authorization".to_string());
        redact_headers.insert("cookie".to_string());
        redact_headers.insert("x-api-key".to_string());
        redact_headers.insert("x-auth-token".to_string());

        Self {
            format: LogOutputFormat::default(),
            include_request_headers: false,
            include_response_headers: false,
            include_request_body: false,
            include_response_body: false,
            max_body_size: 1024,
            correlation_id_header: "x-correlation-id".to_string(),
            generate_correlation_id: true,
            redact_headers,
            include_timing: true,
            service_name: "rustapi".to_string(),
            service_version: None,
            environment: None,
            exclude_paths: vec!["/health".to_string(), "/metrics".to_string()],
            static_fields: Vec::new(),
            log_request_start: true,
            log_request_end: true,
            include_caller_info: false,
        }
    }
}

impl StructuredLoggingConfig {
    /// Create a new builder for StructuredLoggingConfig
    pub fn builder() -> StructuredLoggingConfigBuilder {
        StructuredLoggingConfigBuilder::default()
    }

    /// Create a config optimized for development
    pub fn development() -> Self {
        Self {
            format: LogOutputFormat::Pretty,
            include_request_headers: true,
            include_response_headers: true,
            include_timing: true,
            log_request_start: true,
            log_request_end: true,
            include_caller_info: true,
            ..Default::default()
        }
    }

    /// Create a config optimized for production JSON logging
    pub fn production_json() -> Self {
        Self {
            format: LogOutputFormat::Json,
            include_request_headers: false,
            include_response_headers: false,
            include_timing: true,
            log_request_start: false,
            log_request_end: true,
            include_caller_info: false,
            ..Default::default()
        }
    }

    /// Create a config for Datadog APM integration
    pub fn datadog() -> Self {
        Self {
            format: LogOutputFormat::Datadog,
            include_timing: true,
            log_request_start: false,
            log_request_end: true,
            ..Default::default()
        }
    }

    /// Create a config for Splunk HEC integration
    pub fn splunk() -> Self {
        Self {
            format: LogOutputFormat::Splunk,
            include_timing: true,
            log_request_start: false,
            log_request_end: true,
            ..Default::default()
        }
    }
}

/// Builder for StructuredLoggingConfig
#[derive(Default)]
pub struct StructuredLoggingConfigBuilder {
    config: StructuredLoggingConfig,
}

impl StructuredLoggingConfigBuilder {
    /// Set the output format
    pub fn format(mut self, format: LogOutputFormat) -> Self {
        self.config.format = format;
        self
    }

    /// Set whether to include request headers
    pub fn include_request_headers(mut self, include: bool) -> Self {
        self.config.include_request_headers = include;
        self
    }

    /// Set whether to include response headers
    pub fn include_response_headers(mut self, include: bool) -> Self {
        self.config.include_response_headers = include;
        self
    }

    /// Set whether to include request body
    pub fn include_request_body(mut self, include: bool) -> Self {
        self.config.include_request_body = include;
        self
    }

    /// Set whether to include response body
    pub fn include_response_body(mut self, include: bool) -> Self {
        self.config.include_response_body = include;
        self
    }

    /// Set maximum body size to log
    pub fn max_body_size(mut self, size: usize) -> Self {
        self.config.max_body_size = size;
        self
    }

    /// Set the correlation ID header name
    pub fn correlation_id_header(mut self, header: impl Into<String>) -> Self {
        self.config.correlation_id_header = header.into();
        self
    }

    /// Set whether to generate correlation ID if not present
    pub fn generate_correlation_id(mut self, generate: bool) -> Self {
        self.config.generate_correlation_id = generate;
        self
    }

    /// Add a header to redact
    pub fn redact_header(mut self, header: impl Into<String>) -> Self {
        self.config
            .redact_headers
            .insert(header.into().to_lowercase());
        self
    }

    /// Set headers to redact
    pub fn redact_headers(mut self, headers: Vec<String>) -> Self {
        self.config.redact_headers = headers.into_iter().map(|h| h.to_lowercase()).collect();
        self
    }

    /// Set whether to include timing information
    pub fn include_timing(mut self, include: bool) -> Self {
        self.config.include_timing = include;
        self
    }

    /// Set the service name
    pub fn service_name(mut self, name: impl Into<String>) -> Self {
        self.config.service_name = name.into();
        self
    }

    /// Set the service version
    pub fn service_version(mut self, version: impl Into<String>) -> Self {
        self.config.service_version = Some(version.into());
        self
    }

    /// Set the environment name
    pub fn environment(mut self, env: impl Into<String>) -> Self {
        self.config.environment = Some(env.into());
        self
    }

    /// Add a path to exclude from logging
    pub fn exclude_path(mut self, path: impl Into<String>) -> Self {
        self.config.exclude_paths.push(path.into());
        self
    }

    /// Set paths to exclude
    pub fn exclude_paths(mut self, paths: Vec<String>) -> Self {
        self.config.exclude_paths = paths;
        self
    }

    /// Add a static field to include in all logs
    pub fn static_field(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.config.static_fields.push((key.into(), value.into()));
        self
    }

    /// Set whether to log at request start
    pub fn log_request_start(mut self, log: bool) -> Self {
        self.config.log_request_start = log;
        self
    }

    /// Set whether to log at request end
    pub fn log_request_end(mut self, log: bool) -> Self {
        self.config.log_request_end = log;
        self
    }

    /// Set whether to include caller info
    pub fn include_caller_info(mut self, include: bool) -> Self {
        self.config.include_caller_info = include;
        self
    }

    /// Build the configuration
    pub fn build(self) -> StructuredLoggingConfig {
        self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = StructuredLoggingConfig::default();
        assert_eq!(config.format, LogOutputFormat::Json);
        assert!(config.generate_correlation_id);
        assert!(config.include_timing);
        assert!(config.redact_headers.contains("authorization"));
    }

    #[test]
    fn test_builder() {
        let config = StructuredLoggingConfig::builder()
            .format(LogOutputFormat::Datadog)
            .service_name("my-service")
            .service_version("1.0.0")
            .environment("production")
            .include_request_headers(true)
            .redact_header("x-secret")
            .static_field("region", "us-east-1")
            .build();

        assert_eq!(config.format, LogOutputFormat::Datadog);
        assert_eq!(config.service_name, "my-service");
        assert_eq!(config.service_version, Some("1.0.0".to_string()));
        assert!(config.include_request_headers);
        assert!(config.redact_headers.contains("x-secret"));
        assert_eq!(config.static_fields.len(), 1);
    }

    #[test]
    fn test_development_preset() {
        let config = StructuredLoggingConfig::development();
        assert_eq!(config.format, LogOutputFormat::Pretty);
        assert!(config.include_request_headers);
        assert!(config.include_caller_info);
    }

    #[test]
    fn test_production_preset() {
        let config = StructuredLoggingConfig::production_json();
        assert_eq!(config.format, LogOutputFormat::Json);
        assert!(!config.include_request_headers);
        assert!(!config.log_request_start);
        assert!(config.log_request_end);
    }

    #[test]
    fn test_datadog_preset() {
        let config = StructuredLoggingConfig::datadog();
        assert_eq!(config.format, LogOutputFormat::Datadog);
    }

    #[test]
    fn test_splunk_preset() {
        let config = StructuredLoggingConfig::splunk();
        assert_eq!(config.format, LogOutputFormat::Splunk);
    }
}
