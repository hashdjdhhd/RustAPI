use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::{rngs::OsRng, RngCore};
use std::fmt;

/// A CSRF token.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CsrfToken(String);

impl CsrfToken {
    /// Generate a new random CSRF token of the specified length.
    pub fn generate(length: usize) -> Self {
        let mut bytes = vec![0u8; length];
        OsRng.fill_bytes(&mut bytes);
        let token = URL_SAFE_NO_PAD.encode(&bytes);
        Self(token)
    }

    /// Create a token from an existing string.
    pub fn new(token: String) -> Self {
        Self(token)
    }

    /// Get the token string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for CsrfToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("CsrfToken").field(&"***").finish()
    }
}

impl fmt::Display for CsrfToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl rustapi_core::FromRequestParts for CsrfToken {
    fn from_request_parts(req: &rustapi_core::Request) -> rustapi_core::Result<Self> {
        use http::StatusCode;
        use rustapi_core::ApiError;

        match req.extensions().get::<CsrfToken>() {
            Some(token) => Ok(token.clone()),
            None => Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "csrf_missing",
                "CSRF token missing from request extensions. Ensure CSRF middleware is enabled.",
            )),
        }
    }
}

impl rustapi_openapi::OperationModifier for CsrfToken {
    fn update_operation(_op: &mut rustapi_openapi::Operation) {
        // CSRF token is handled by middleware, so we don't need to document
        // it as a parameter for every operation that extracts it.
        // It's usually part of the global security requirements.
    }
}
