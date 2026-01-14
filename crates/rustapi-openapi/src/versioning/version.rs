//! API Version type and parsing
//!
//! Provides semantic versioning support for API versions.

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

/// API version using semantic versioning
///
/// Supports formats like:
/// - `v1`, `v2` (major only)
/// - `v1.0`, `v1.2` (major.minor)
/// - `v1.0.0`, `v1.2.3` (major.minor.patch)
/// - `1`, `1.0`, `1.0.0` (without 'v' prefix)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ApiVersion {
    /// Major version number
    pub major: u32,
    /// Minor version number (defaults to 0)
    pub minor: u32,
    /// Patch version number (defaults to 0)
    pub patch: u32,
}

impl ApiVersion {
    /// Create a new version
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Create a version with only major number
    pub fn major(major: u32) -> Self {
        Self {
            major,
            minor: 0,
            patch: 0,
        }
    }

    /// Create version 1.0.0
    pub fn v1() -> Self {
        Self::new(1, 0, 0)
    }

    /// Create version 2.0.0
    pub fn v2() -> Self {
        Self::new(2, 0, 0)
    }

    /// Create version 3.0.0
    pub fn v3() -> Self {
        Self::new(3, 0, 0)
    }

    /// Check if this version is compatible with another version
    ///
    /// Uses semantic versioning compatibility rules:
    /// - Same major version is considered compatible
    pub fn is_compatible_with(&self, other: &ApiVersion) -> bool {
        self.major == other.major
    }

    /// Check if this version satisfies a version range
    pub fn satisfies(&self, range: &VersionRange) -> bool {
        range.contains(self)
    }

    /// Format as path segment (e.g., "v1", "v1.2")
    pub fn as_path_segment(&self) -> String {
        if self.minor == 0 && self.patch == 0 {
            format!("v{}", self.major)
        } else if self.patch == 0 {
            format!("v{}.{}", self.major, self.minor)
        } else {
            format!("v{}.{}.{}", self.major, self.minor, self.patch)
        }
    }
}

impl Default for ApiVersion {
    fn default() -> Self {
        Self::v1()
    }
}

impl fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromStr for ApiVersion {
    type Err = VersionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Remove optional 'v' prefix
        let s = s
            .strip_prefix('v')
            .or_else(|| s.strip_prefix('V'))
            .unwrap_or(s);

        let parts: Vec<&str> = s.split('.').collect();

        match parts.len() {
            1 => {
                let major = parts[0]
                    .parse()
                    .map_err(|_| VersionParseError::InvalidNumber)?;
                Ok(ApiVersion::major(major))
            }
            2 => {
                let major = parts[0]
                    .parse()
                    .map_err(|_| VersionParseError::InvalidNumber)?;
                let minor = parts[1]
                    .parse()
                    .map_err(|_| VersionParseError::InvalidNumber)?;
                Ok(ApiVersion::new(major, minor, 0))
            }
            3 => {
                let major = parts[0]
                    .parse()
                    .map_err(|_| VersionParseError::InvalidNumber)?;
                let minor = parts[1]
                    .parse()
                    .map_err(|_| VersionParseError::InvalidNumber)?;
                let patch = parts[2]
                    .parse()
                    .map_err(|_| VersionParseError::InvalidNumber)?;
                Ok(ApiVersion::new(major, minor, patch))
            }
            _ => Err(VersionParseError::InvalidFormat),
        }
    }
}

impl PartialOrd for ApiVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ApiVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.major.cmp(&other.major) {
            Ordering::Equal => match self.minor.cmp(&other.minor) {
                Ordering::Equal => self.patch.cmp(&other.patch),
                ord => ord,
            },
            ord => ord,
        }
    }
}

/// Error type for version parsing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionParseError {
    /// Invalid number in version string
    InvalidNumber,
    /// Invalid version format
    InvalidFormat,
    /// Empty version string
    Empty,
}

impl fmt::Display for VersionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNumber => write!(f, "invalid number in version"),
            Self::InvalidFormat => write!(f, "invalid version format"),
            Self::Empty => write!(f, "empty version string"),
        }
    }
}

impl std::error::Error for VersionParseError {}

/// Version range for matching multiple versions
#[derive(Debug, Clone)]
pub struct VersionRange {
    /// Minimum version (inclusive)
    pub min: Option<ApiVersion>,
    /// Maximum version (inclusive)
    pub max: Option<ApiVersion>,
    /// Specific excluded versions
    pub excluded: Vec<ApiVersion>,
}

impl VersionRange {
    /// Create a new range with no constraints
    pub fn any() -> Self {
        Self {
            min: None,
            max: None,
            excluded: Vec::new(),
        }
    }

    /// Create a range for a specific major version
    pub fn major(version: u32) -> Self {
        Self {
            min: Some(ApiVersion::new(version, 0, 0)),
            max: Some(ApiVersion::new(version, u32::MAX, u32::MAX)),
            excluded: Vec::new(),
        }
    }

    /// Create a range from a minimum version (inclusive)
    pub fn from(version: ApiVersion) -> Self {
        Self {
            min: Some(version),
            max: None,
            excluded: Vec::new(),
        }
    }

    /// Create a range up to a maximum version (inclusive)
    pub fn until(version: ApiVersion) -> Self {
        Self {
            min: None,
            max: Some(version),
            excluded: Vec::new(),
        }
    }

    /// Create a range between two versions (inclusive)
    pub fn between(min: ApiVersion, max: ApiVersion) -> Self {
        Self {
            min: Some(min),
            max: Some(max),
            excluded: Vec::new(),
        }
    }

    /// Create a range for exactly one version
    pub fn exact(version: ApiVersion) -> Self {
        Self {
            min: Some(version),
            max: Some(version),
            excluded: Vec::new(),
        }
    }

    /// Exclude a specific version from the range
    pub fn exclude(mut self, version: ApiVersion) -> Self {
        self.excluded.push(version);
        self
    }

    /// Check if a version is within this range
    pub fn contains(&self, version: &ApiVersion) -> bool {
        // Check exclusions first
        if self.excluded.contains(version) {
            return false;
        }

        // Check minimum bound
        if let Some(min) = &self.min {
            if version < min {
                return false;
            }
        }

        // Check maximum bound
        if let Some(max) = &self.max {
            if version > max {
                return false;
            }
        }

        true
    }
}

impl Default for VersionRange {
    fn default() -> Self {
        Self::any()
    }
}

/// Matcher for version selection
pub trait VersionMatcher: Send + Sync {
    /// Check if a version matches
    fn matches(&self, version: &ApiVersion) -> bool;

    /// Get the priority (higher = preferred)
    fn priority(&self) -> i32 {
        0
    }
}

impl VersionMatcher for ApiVersion {
    fn matches(&self, version: &ApiVersion) -> bool {
        self == version
    }
}

impl VersionMatcher for VersionRange {
    fn matches(&self, version: &ApiVersion) -> bool {
        self.contains(version)
    }
}

/// Matcher for major version only
pub struct MajorVersionMatcher {
    major: u32,
}

impl MajorVersionMatcher {
    /// Create a matcher for a specific major version
    pub fn new(major: u32) -> Self {
        Self { major }
    }
}

impl VersionMatcher for MajorVersionMatcher {
    fn matches(&self, version: &ApiVersion) -> bool {
        version.major == self.major
    }
}

/// Matcher that accepts any version
pub struct AnyVersionMatcher;

impl VersionMatcher for AnyVersionMatcher {
    fn matches(&self, _version: &ApiVersion) -> bool {
        true
    }

    fn priority(&self) -> i32 {
        -1 // Lower priority than specific matchers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parsing() {
        assert_eq!("1".parse::<ApiVersion>().unwrap(), ApiVersion::major(1));
        assert_eq!("v1".parse::<ApiVersion>().unwrap(), ApiVersion::major(1));
        assert_eq!(
            "1.2".parse::<ApiVersion>().unwrap(),
            ApiVersion::new(1, 2, 0)
        );
        assert_eq!(
            "v1.2.3".parse::<ApiVersion>().unwrap(),
            ApiVersion::new(1, 2, 3)
        );
        assert_eq!("V2".parse::<ApiVersion>().unwrap(), ApiVersion::major(2));
    }

    #[test]
    fn test_version_parsing_errors() {
        assert!("".parse::<ApiVersion>().is_err());
        assert!("x".parse::<ApiVersion>().is_err());
        assert!("1.2.3.4".parse::<ApiVersion>().is_err());
        assert!("v".parse::<ApiVersion>().is_err());
    }

    #[test]
    fn test_version_comparison() {
        assert!(ApiVersion::new(2, 0, 0) > ApiVersion::new(1, 0, 0));
        assert!(ApiVersion::new(1, 1, 0) > ApiVersion::new(1, 0, 0));
        assert!(ApiVersion::new(1, 0, 1) > ApiVersion::new(1, 0, 0));
        assert!(ApiVersion::new(1, 0, 0) == ApiVersion::new(1, 0, 0));
    }

    #[test]
    fn test_version_compatibility() {
        let v1_0 = ApiVersion::new(1, 0, 0);
        let v1_1 = ApiVersion::new(1, 1, 0);
        let v2_0 = ApiVersion::new(2, 0, 0);

        assert!(v1_0.is_compatible_with(&v1_1));
        assert!(v1_1.is_compatible_with(&v1_0));
        assert!(!v1_0.is_compatible_with(&v2_0));
    }

    #[test]
    fn test_version_as_path_segment() {
        assert_eq!(ApiVersion::major(1).as_path_segment(), "v1");
        assert_eq!(ApiVersion::new(1, 2, 0).as_path_segment(), "v1.2");
        assert_eq!(ApiVersion::new(1, 2, 3).as_path_segment(), "v1.2.3");
    }

    #[test]
    fn test_version_range_contains() {
        let range = VersionRange::between(ApiVersion::new(1, 0, 0), ApiVersion::new(2, 0, 0));

        assert!(range.contains(&ApiVersion::new(1, 0, 0)));
        assert!(range.contains(&ApiVersion::new(1, 5, 0)));
        assert!(range.contains(&ApiVersion::new(2, 0, 0)));
        assert!(!range.contains(&ApiVersion::new(0, 9, 0)));
        assert!(!range.contains(&ApiVersion::new(2, 0, 1)));
    }

    #[test]
    fn test_version_range_exclude() {
        let range = VersionRange::major(1).exclude(ApiVersion::new(1, 5, 0));

        assert!(range.contains(&ApiVersion::new(1, 0, 0)));
        assert!(range.contains(&ApiVersion::new(1, 4, 0)));
        assert!(!range.contains(&ApiVersion::new(1, 5, 0)));
        assert!(range.contains(&ApiVersion::new(1, 6, 0)));
    }

    #[test]
    fn test_version_display() {
        assert_eq!(ApiVersion::new(1, 2, 3).to_string(), "1.2.3");
    }
}
