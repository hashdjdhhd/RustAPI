//! OpenTelemetry configuration types

use std::time::Duration;

/// Exporter type for OpenTelemetry traces
#[derive(Clone, Debug, Default)]
pub enum OtelExporter {
    /// OTLP gRPC exporter (default)
    #[default]
    OtlpGrpc,
    /// OTLP HTTP exporter
    OtlpHttp,
    /// Jaeger exporter
    Jaeger,
    /// Zipkin exporter
    Zipkin,
    /// Console exporter (for debugging)
    Console,
    /// No-op exporter (disabled)
    None,
}

/// Trace sampling strategy
#[derive(Clone, Debug, Default)]
pub enum TraceSampler {
    /// Always sample all traces
    #[default]
    AlwaysOn,
    /// Never sample traces
    AlwaysOff,
    /// Sample a ratio of traces (0.0 - 1.0)
    TraceIdRatio(f64),
    /// Sample based on parent span decision
    ParentBased,
}

/// OpenTelemetry configuration
#[derive(Clone, Debug)]
pub struct OtelConfig {
    /// Service name for traces
    pub service_name: String,
    /// Service version
    pub service_version: Option<String>,
    /// Service namespace
    pub service_namespace: Option<String>,
    /// Deployment environment (e.g., "production", "staging")
    pub deployment_environment: Option<String>,
    /// OTLP endpoint URL
    pub endpoint: Option<String>,
    /// Exporter type
    pub exporter: OtelExporter,
    /// Trace sampler configuration
    pub sampler: TraceSampler,
    /// Export timeout
    pub export_timeout: Duration,
    /// Export interval for batch exporter
    pub export_interval: Duration,
    /// Maximum queue size for batch exporter
    pub max_queue_size: usize,
    /// Maximum export batch size
    pub max_export_batch_size: usize,
    /// Whether to enable metrics collection
    pub enable_metrics: bool,
    /// Whether to propagate W3C trace context
    pub propagate_context: bool,
    /// Additional resource attributes
    pub resource_attributes: Vec<(String, String)>,
    /// Headers to include in traces
    pub trace_headers: Vec<String>,
    /// Paths to exclude from tracing
    pub exclude_paths: Vec<String>,
}

impl Default for OtelConfig {
    fn default() -> Self {
        Self {
            service_name: "rustapi-service".to_string(),
            service_version: None,
            service_namespace: None,
            deployment_environment: None,
            endpoint: None,
            exporter: OtelExporter::default(),
            sampler: TraceSampler::default(),
            export_timeout: Duration::from_secs(30),
            export_interval: Duration::from_secs(5),
            max_queue_size: 2048,
            max_export_batch_size: 512,
            enable_metrics: true,
            propagate_context: true,
            resource_attributes: Vec::new(),
            trace_headers: vec![
                "user-agent".to_string(),
                "content-type".to_string(),
                "x-request-id".to_string(),
            ],
            exclude_paths: vec!["/health".to_string(), "/metrics".to_string()],
        }
    }
}

impl OtelConfig {
    /// Create a new OtelConfig builder
    pub fn builder() -> OtelConfigBuilder {
        OtelConfigBuilder::default()
    }
}

/// Builder for OtelConfig
#[derive(Default)]
pub struct OtelConfigBuilder {
    config: OtelConfig,
}

impl OtelConfigBuilder {
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

    /// Set the service namespace
    pub fn service_namespace(mut self, namespace: impl Into<String>) -> Self {
        self.config.service_namespace = Some(namespace.into());
        self
    }

    /// Set the deployment environment
    pub fn deployment_environment(mut self, env: impl Into<String>) -> Self {
        self.config.deployment_environment = Some(env.into());
        self
    }

    /// Set the OTLP endpoint URL
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.config.endpoint = Some(endpoint.into());
        self
    }

    /// Set the exporter type
    pub fn exporter(mut self, exporter: OtelExporter) -> Self {
        self.config.exporter = exporter;
        self
    }

    /// Set the trace sampler
    pub fn sampler(mut self, sampler: TraceSampler) -> Self {
        self.config.sampler = sampler;
        self
    }

    /// Set the export timeout
    pub fn export_timeout(mut self, timeout: Duration) -> Self {
        self.config.export_timeout = timeout;
        self
    }

    /// Set the export interval
    pub fn export_interval(mut self, interval: Duration) -> Self {
        self.config.export_interval = interval;
        self
    }

    /// Set the maximum queue size
    pub fn max_queue_size(mut self, size: usize) -> Self {
        self.config.max_queue_size = size;
        self
    }

    /// Set the maximum export batch size
    pub fn max_export_batch_size(mut self, size: usize) -> Self {
        self.config.max_export_batch_size = size;
        self
    }

    /// Enable or disable metrics collection
    pub fn enable_metrics(mut self, enabled: bool) -> Self {
        self.config.enable_metrics = enabled;
        self
    }

    /// Enable or disable context propagation
    pub fn propagate_context(mut self, enabled: bool) -> Self {
        self.config.propagate_context = enabled;
        self
    }

    /// Add a resource attribute
    pub fn resource_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.config
            .resource_attributes
            .push((key.into(), value.into()));
        self
    }

    /// Add multiple resource attributes
    pub fn resource_attributes(mut self, attrs: Vec<(String, String)>) -> Self {
        self.config.resource_attributes.extend(attrs);
        self
    }

    /// Add a header to trace
    pub fn trace_header(mut self, header: impl Into<String>) -> Self {
        self.config.trace_headers.push(header.into());
        self
    }

    /// Add multiple headers to trace
    pub fn trace_headers(mut self, headers: Vec<String>) -> Self {
        self.config.trace_headers.extend(headers);
        self
    }

    /// Add a path to exclude from tracing
    pub fn exclude_path(mut self, path: impl Into<String>) -> Self {
        self.config.exclude_paths.push(path.into());
        self
    }

    /// Add multiple paths to exclude
    pub fn exclude_paths(mut self, paths: Vec<String>) -> Self {
        self.config.exclude_paths.extend(paths);
        self
    }

    /// Build the OtelConfig
    pub fn build(self) -> OtelConfig {
        self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = OtelConfig::default();
        assert_eq!(config.service_name, "rustapi-service");
        assert!(config.propagate_context);
        assert!(config.enable_metrics);
    }

    #[test]
    fn test_builder() {
        let config = OtelConfig::builder()
            .service_name("my-service")
            .service_version("1.0.0")
            .endpoint("http://localhost:4317")
            .exporter(OtelExporter::OtlpGrpc)
            .sampler(TraceSampler::TraceIdRatio(0.5))
            .resource_attribute("env", "production")
            .exclude_path("/ready")
            .build();

        assert_eq!(config.service_name, "my-service");
        assert_eq!(config.service_version, Some("1.0.0".to_string()));
        assert_eq!(config.endpoint, Some("http://localhost:4317".to_string()));
        assert_eq!(config.resource_attributes.len(), 1);
        assert!(config.exclude_paths.contains(&"/ready".to_string()));
    }

    #[test]
    fn test_sampler_default() {
        let sampler = TraceSampler::default();
        matches!(sampler, TraceSampler::AlwaysOn);
    }
}
