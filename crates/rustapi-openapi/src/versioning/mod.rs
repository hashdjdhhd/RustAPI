//! API Versioning support for OpenAPI
//!
//! This module provides comprehensive API versioning capabilities including:
//!
//! - Version parsing and comparison
//! - Multiple versioning strategies (path, header, query, accept header)
//! - Version-based routing
//! - Separate OpenAPI spec generation per version
//!
//! # Example
//!
//! ```rust,ignore
//! use rustapi_openapi::versioning::{ApiVersion, VersionStrategy, VersionRouter};
//!
//! let router = VersionRouter::new()
//!     .strategy(VersionStrategy::Path("/v{version}"))
//!     .default_version(ApiVersion::new(1, 0, 0))
//!     .version(ApiVersion::new(1, 0, 0), v1_routes)
//!     .version(ApiVersion::new(2, 0, 0), v2_routes);
//! ```

mod router;
mod strategy;
mod version;

#[cfg(test)]
mod tests;

pub use router::{
    DeprecationInfo, ResolvedVersion, VersionFallback, VersionRouter, VersionedRouteConfig,
    VersionedSpecBuilder,
};
pub use strategy::{ExtractedVersion, VersionExtractor, VersionSource, VersionStrategy};
pub use version::{
    AnyVersionMatcher, ApiVersion, MajorVersionMatcher, VersionMatcher, VersionParseError,
    VersionRange,
};
