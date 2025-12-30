//! CORS (Cross-Origin Resource Sharing) middleware.
//!
//! This module provides configurable CORS middleware with a builder pattern
//! for controlling cross-origin access to your API.
//!
//! # Example
//!
//! ```ignore
//! use rustapi_extras::cors::CorsLayer;
//!
//! let cors = CorsLayer::new()
//!     .allow_origins(["https://example.com"])
//!     .allow_methods([Method::GET, Method::POST])
//!     .allow_credentials(true);
//! ```

use http::Method;
use std::time::Duration;

/// Specifies which origins are allowed for CORS requests.
#[derive(Debug, Clone)]
pub enum AllowedOrigins {
    /// Allow any origin (`Access-Control-Allow-Origin: *`).
    Any,
    /// Allow only specific origins.
    List(Vec<String>),
}

impl Default for AllowedOrigins {
    fn default() -> Self {
        Self::List(Vec::new())
    }
}

/// CORS middleware layer with builder pattern configuration.
///
/// Handles preflight OPTIONS requests and adds appropriate CORS headers
/// to responses.
#[derive(Debug, Clone)]
pub struct CorsLayer {
    origins: AllowedOrigins,
    methods: Vec<Method>,
    headers: Vec<String>,
    credentials: bool,
    max_age: Option<Duration>,
}

impl Default for CorsLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl CorsLayer {
    /// Create a new CORS layer with restrictive defaults.
    pub fn new() -> Self {
        Self {
            origins: AllowedOrigins::default(),
            methods: vec![Method::GET, Method::HEAD, Method::OPTIONS],
            headers: Vec::new(),
            credentials: false,
            max_age: None,
        }
    }

    /// Create a permissive CORS layer that allows everything.
    ///
    /// This is useful for development but should be used with caution
    /// in production.
    pub fn permissive() -> Self {
        Self {
            origins: AllowedOrigins::Any,
            methods: vec![
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::PATCH,
                Method::DELETE,
                Method::HEAD,
                Method::OPTIONS,
            ],
            headers: vec!["*".to_string()],
            credentials: false,
            max_age: Some(Duration::from_secs(86400)),
        }
    }

    /// Create a restrictive CORS layer with minimal permissions.
    pub fn restrictive() -> Self {
        Self::new()
    }

    /// Allow any origin.
    pub fn allow_any_origin(mut self) -> Self {
        self.origins = AllowedOrigins::Any;
        self
    }

    /// Allow specific origins.
    pub fn allow_origins<I, S>(mut self, origins: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.origins = AllowedOrigins::List(origins.into_iter().map(Into::into).collect());
        self
    }

    /// Allow specific HTTP methods.
    pub fn allow_methods<I>(mut self, methods: I) -> Self
    where
        I: IntoIterator<Item = Method>,
    {
        self.methods = methods.into_iter().collect();
        self
    }

    /// Allow specific headers.
    pub fn allow_headers<I, S>(mut self, headers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.headers = headers.into_iter().map(Into::into).collect();
        self
    }

    /// Allow credentials (cookies, authorization headers).
    pub fn allow_credentials(mut self, allow: bool) -> Self {
        self.credentials = allow;
        self
    }

    /// Set the max age for preflight cache.
    pub fn max_age(mut self, duration: Duration) -> Self {
        self.max_age = Some(duration);
        self
    }

    /// Get the configured origins.
    pub fn origins(&self) -> &AllowedOrigins {
        &self.origins
    }

    /// Get the configured methods.
    pub fn methods(&self) -> &[Method] {
        &self.methods
    }

    /// Get the configured headers.
    pub fn headers(&self) -> &[String] {
        &self.headers
    }

    /// Check if credentials are allowed.
    pub fn credentials(&self) -> bool {
        self.credentials
    }

    /// Get the max age configuration.
    pub fn max_age_duration(&self) -> Option<Duration> {
        self.max_age
    }
}
