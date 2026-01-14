//! Version extraction strategies
//!
//! Provides different strategies for extracting API versions from requests.

use super::version::ApiVersion;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Strategy for extracting API version from requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VersionStrategy {
    /// Extract version from URL path (e.g., /v1/users)
    ///
    /// The pattern should include `{version}` placeholder
    /// Example: "/v{version}/" or "/{version}/"
    Path {
        /// Pattern for matching version in path
        pattern: String,
    },

    /// Extract version from HTTP header
    ///
    /// Example: X-API-Version: 1.0
    Header {
        /// Header name to read version from
        name: String,
    },

    /// Extract version from query parameter
    ///
    /// Example: ?version=1.0 or ?api-version=1.0
    Query {
        /// Query parameter name
        param: String,
    },

    /// Extract version from Accept header media type
    ///
    /// Example: Accept: application/vnd.api.v1+json
    Accept {
        /// Media type pattern with version placeholder
        /// Example: "application/vnd.{vendor}.v{version}+json"
        pattern: String,
    },

    /// Use a custom extractor function
    ///
    /// Uses a named custom extractor registered with the router
    Custom {
        /// Name of the custom extractor
        name: String,
    },
}

impl VersionStrategy {
    /// Create a path-based versioning strategy
    ///
    /// Default pattern: "/v{version}/"
    pub fn path() -> Self {
        Self::Path {
            pattern: "/v{version}/".to_string(),
        }
    }

    /// Create a path strategy with custom pattern
    pub fn path_with_pattern(pattern: impl Into<String>) -> Self {
        Self::Path {
            pattern: pattern.into(),
        }
    }

    /// Create a header-based versioning strategy
    ///
    /// Default header: "X-API-Version"
    pub fn header() -> Self {
        Self::Header {
            name: "X-API-Version".to_string(),
        }
    }

    /// Create a header strategy with custom header name
    pub fn header_with_name(name: impl Into<String>) -> Self {
        Self::Header { name: name.into() }
    }

    /// Create a query parameter versioning strategy
    ///
    /// Default parameter: "version"
    pub fn query() -> Self {
        Self::Query {
            param: "version".to_string(),
        }
    }

    /// Create a query strategy with custom parameter name
    pub fn query_with_param(param: impl Into<String>) -> Self {
        Self::Query {
            param: param.into(),
        }
    }

    /// Create an Accept header versioning strategy
    ///
    /// Default pattern: "application/vnd.api.v{version}+json"
    pub fn accept() -> Self {
        Self::Accept {
            pattern: "application/vnd.api.v{version}+json".to_string(),
        }
    }

    /// Create an Accept strategy with custom pattern
    pub fn accept_with_pattern(pattern: impl Into<String>) -> Self {
        Self::Accept {
            pattern: pattern.into(),
        }
    }

    /// Create a custom extraction strategy
    pub fn custom(name: impl Into<String>) -> Self {
        Self::Custom { name: name.into() }
    }
}

impl Default for VersionStrategy {
    fn default() -> Self {
        Self::path()
    }
}

/// Version extractor that can extract versions from request data
#[derive(Debug, Clone)]
pub struct VersionExtractor {
    /// Strategies to try in order
    strategies: Vec<VersionStrategy>,
    /// Default version if none can be extracted
    default: ApiVersion,
}

impl VersionExtractor {
    /// Create a new extractor with default settings
    pub fn new() -> Self {
        Self {
            strategies: vec![VersionStrategy::path()],
            default: ApiVersion::v1(),
        }
    }

    /// Create an extractor with a single strategy
    pub fn with_strategy(strategy: VersionStrategy) -> Self {
        Self {
            strategies: vec![strategy],
            default: ApiVersion::v1(),
        }
    }

    /// Create an extractor with multiple strategies (tried in order)
    pub fn with_strategies(strategies: Vec<VersionStrategy>) -> Self {
        Self {
            strategies,
            default: ApiVersion::v1(),
        }
    }

    /// Set the default version
    pub fn default_version(mut self, version: ApiVersion) -> Self {
        self.default = version;
        self
    }

    /// Add a strategy to try
    pub fn add_strategy(mut self, strategy: VersionStrategy) -> Self {
        self.strategies.push(strategy);
        self
    }

    /// Extract version from path
    pub fn extract_from_path(&self, path: &str) -> Option<ApiVersion> {
        for strategy in &self.strategies {
            if let VersionStrategy::Path { pattern } = strategy {
                if let Some(version) = Self::extract_path_version(path, pattern) {
                    return Some(version);
                }
            }
        }
        None
    }

    /// Extract version from headers
    pub fn extract_from_headers(&self, headers: &HashMap<String, String>) -> Option<ApiVersion> {
        for strategy in &self.strategies {
            match strategy {
                VersionStrategy::Header { name } => {
                    if let Some(value) = headers.get(&name.to_lowercase()) {
                        if let Ok(version) = value.parse() {
                            return Some(version);
                        }
                    }
                }
                VersionStrategy::Accept { pattern } => {
                    if let Some(accept) = headers.get("accept") {
                        if let Some(version) = Self::extract_accept_version(accept, pattern) {
                            return Some(version);
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }

    /// Extract version from query string
    pub fn extract_from_query(&self, query: &str) -> Option<ApiVersion> {
        let params: HashMap<_, _> = query
            .split('&')
            .filter_map(|pair| {
                let mut parts = pair.splitn(2, '=');
                Some((parts.next()?.to_string(), parts.next()?.to_string()))
            })
            .collect();

        for strategy in &self.strategies {
            if let VersionStrategy::Query { param } = strategy {
                if let Some(value) = params.get(param) {
                    if let Ok(version) = value.parse() {
                        return Some(version);
                    }
                }
            }
        }
        None
    }

    /// Get the default version
    pub fn get_default(&self) -> ApiVersion {
        self.default
    }

    /// Extract version from path using pattern
    fn extract_path_version(path: &str, pattern: &str) -> Option<ApiVersion> {
        // Find the version placeholder position
        let before = pattern.split("{version}").next()?;
        let after = pattern.split("{version}").nth(1)?;

        // Find the version segment in the path
        if let Some(start) = path.find(before) {
            let version_start = start + before.len();
            let remaining = &path[version_start..];

            // Find the end of the version segment
            let version_end = if after.is_empty() {
                remaining.len()
            } else {
                remaining.find(after).unwrap_or(remaining.len())
            };

            let version_str = &remaining[..version_end];
            version_str.parse().ok()
        } else {
            None
        }
    }

    /// Extract version from Accept header
    fn extract_accept_version(accept: &str, pattern: &str) -> Option<ApiVersion> {
        // Parse the pattern
        let before = pattern.split("{version}").next()?;
        let after = pattern.split("{version}").nth(1)?;

        // Find in accept header
        for media_type in accept.split(',').map(|s| s.trim()) {
            if let Some(start) = media_type.find(before) {
                let version_start = start + before.len();
                let remaining = &media_type[version_start..];

                let version_end = if after.is_empty() {
                    remaining.len()
                } else {
                    remaining.find(after).unwrap_or(remaining.len())
                };

                let version_str = &remaining[..version_end];
                if let Ok(version) = version_str.parse() {
                    return Some(version);
                }
            }
        }
        None
    }

    /// Remove version from path, returning the path without version prefix/suffix
    pub fn strip_version_from_path(&self, path: &str) -> String {
        for strategy in &self.strategies {
            if let VersionStrategy::Path { pattern } = strategy {
                if let Some(stripped) = Self::strip_path_version(path, pattern) {
                    return stripped;
                }
            }
        }
        path.to_string()
    }

    /// Strip version from path using pattern
    fn strip_path_version(path: &str, pattern: &str) -> Option<String> {
        let before = pattern.split("{version}").next()?;
        let after = pattern.split("{version}").nth(1)?;

        if let Some(start) = path.find(before) {
            let version_start = start + before.len();
            let remaining = &path[version_start..];

            let version_end = if after.is_empty() {
                remaining.len()
            } else {
                remaining.find(after)?
            };

            // Verify it's a valid version
            let version_str = &remaining[..version_end];
            if version_str.parse::<ApiVersion>().is_ok() {
                let prefix = &path[..start];
                // The suffix starts after version_end + after.len() in remaining
                // But we want to keep the leading / for paths
                let suffix = &remaining[version_end + after.len()..];
                // Ensure result starts with / if original path did and prefix is empty
                if path.starts_with('/') && prefix.is_empty() && !suffix.starts_with('/') {
                    return Some(format!("/{}", suffix));
                }
                return Some(format!("{}{}", prefix, suffix));
            }
        }
        None
    }
}

impl Default for VersionExtractor {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of version extraction
#[derive(Debug, Clone)]
pub struct ExtractedVersion {
    /// The extracted version
    pub version: ApiVersion,
    /// Source of the version
    pub source: VersionSource,
}

/// Source from which version was extracted
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VersionSource {
    /// Extracted from URL path
    Path,
    /// Extracted from HTTP header
    Header,
    /// Extracted from query parameter
    Query,
    /// Extracted from Accept header
    Accept,
    /// Default version was used
    Default,
    /// Custom extraction
    Custom,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_from_path() {
        let extractor = VersionExtractor::new();

        assert_eq!(
            extractor.extract_from_path("/v1/users"),
            Some(ApiVersion::major(1))
        );
        assert_eq!(
            extractor.extract_from_path("/v2/products/123"),
            Some(ApiVersion::major(2))
        );
        assert_eq!(
            extractor.extract_from_path("/v1.2/items"),
            Some(ApiVersion::new(1, 2, 0))
        );
    }

    #[test]
    fn test_extract_from_header() {
        let extractor = VersionExtractor::with_strategy(VersionStrategy::header());
        let mut headers = HashMap::new();
        headers.insert("x-api-version".to_string(), "2.0".to_string());

        assert_eq!(
            extractor.extract_from_headers(&headers),
            Some(ApiVersion::new(2, 0, 0))
        );
    }

    #[test]
    fn test_extract_from_query() {
        let extractor = VersionExtractor::with_strategy(VersionStrategy::query());

        assert_eq!(
            extractor.extract_from_query("version=1&other=value"),
            Some(ApiVersion::major(1))
        );
        assert_eq!(
            extractor.extract_from_query("foo=bar&version=2.1"),
            Some(ApiVersion::new(2, 1, 0))
        );
    }

    #[test]
    fn test_extract_from_accept() {
        let extractor = VersionExtractor::with_strategy(VersionStrategy::accept());
        let mut headers = HashMap::new();
        headers.insert(
            "accept".to_string(),
            "application/vnd.api.v2+json".to_string(),
        );

        assert_eq!(
            extractor.extract_from_headers(&headers),
            Some(ApiVersion::major(2))
        );
    }

    #[test]
    fn test_strip_version_from_path() {
        let extractor = VersionExtractor::new();

        assert_eq!(extractor.strip_version_from_path("/v1/users"), "/users");
        assert_eq!(
            extractor.strip_version_from_path("/v2.0/products/123"),
            "/products/123"
        );
    }

    #[test]
    fn test_multiple_strategies() {
        let extractor = VersionExtractor::with_strategies(vec![
            VersionStrategy::path(),
            VersionStrategy::header(),
            VersionStrategy::query(),
        ])
        .default_version(ApiVersion::v1());

        // Path takes precedence
        assert_eq!(
            extractor.extract_from_path("/v2/test"),
            Some(ApiVersion::major(2))
        );

        // Falls back to query
        assert_eq!(
            extractor.extract_from_query("version=3"),
            Some(ApiVersion::major(3))
        );
    }

    #[test]
    fn test_custom_path_pattern() {
        let extractor =
            VersionExtractor::with_strategy(VersionStrategy::path_with_pattern("/api/{version}/"));

        assert_eq!(
            extractor.extract_from_path("/api/1/users"),
            Some(ApiVersion::major(1))
        );
        assert_eq!(
            extractor.extract_from_path("/api/2.0/products"),
            Some(ApiVersion::new(2, 0, 0))
        );
    }
}
