use cookie::SameSite;
use std::time::Duration;

/// Configuration for CSRF protection.
#[derive(Clone, Debug)]
pub struct CsrfConfig {
    /// The name of the cookie used to store the CSRF token.
    /// Default: "XSRF-TOKEN"
    pub cookie_name: String,

    /// The name of the header expected to contain the CSRF token.
    /// Default: "X-XSRF-TOKEN"
    pub header_name: String,

    /// The path for the CSRF cookie.
    /// Default: "/"
    pub cookie_path: String,

    /// The domain for the CSRF cookie.
    /// Default: None
    pub cookie_domain: Option<String>,

    /// Whether the CSRF cookie should be secure (HTTPS only).
    /// Default: true (in release mode)
    pub cookie_secure: bool,

    /// Whether the CSRF cookie should be HTTP Only.
    /// For the Double-Submit Cookie pattern, this MUST be false so the client can read it
    /// and send it back in a header.
    /// Default: false
    pub cookie_http_only: bool,

    /// The SameSite attribute for the CSRF cookie.
    /// Default: Lax
    pub cookie_same_site: SameSite,

    /// The lifetime of the CSRF cookie.
    /// Default: 24 hours
    pub cookie_max_age: Duration,

    /// The length of the generated random token (in bytes).
    /// Default: 32 (resulting in ~44 chars base64)
    pub token_length: usize,
}

impl Default for CsrfConfig {
    fn default() -> Self {
        Self {
            cookie_name: "XSRF-TOKEN".to_string(),
            header_name: "X-XSRF-TOKEN".to_string(),
            cookie_path: "/".to_string(),
            cookie_domain: None,
            cookie_secure: true, // Should logic check generic debug/release?
            cookie_http_only: false,
            cookie_same_site: SameSite::Lax,
            cookie_max_age: Duration::from_secs(60 * 60 * 24),
            token_length: 32,
        }
    }
}

impl CsrfConfig {
    /// Create a new default configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the cookie name.
    pub fn cookie_name(mut self, name: impl Into<String>) -> Self {
        self.cookie_name = name.into();
        self
    }

    /// Set the header name.
    pub fn header_name(mut self, name: impl Into<String>) -> Self {
        self.header_name = name.into();
        self
    }

    /// Set the cookie domain.
    pub fn cookie_domain(mut self, domain: impl Into<String>) -> Self {
        self.cookie_domain = Some(domain.into());
        self
    }

    /// Set the secure flag.
    pub fn secure(mut self, secure: bool) -> Self {
        self.cookie_secure = secure;
        self
    }

    /// Set the SameSite attribute.
    pub fn same_site(mut self, same_site: SameSite) -> Self {
        self.cookie_same_site = same_site;
        self
    }
}
