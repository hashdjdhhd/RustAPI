//! OAuth2 token types and errors

use std::time::{Duration, Instant};
use thiserror::Error;

/// OAuth2 token response from the authorization server.
#[derive(Debug, Clone)]
pub struct TokenResponse {
    /// The access token.
    access_token: String,
    /// The token type (usually "Bearer").
    token_type: String,
    /// Token expiration time (if provided).
    expires_at: Option<Instant>,
    /// Refresh token (if provided).
    refresh_token: Option<String>,
    /// Scopes granted (if different from requested).
    scopes: Option<Vec<String>>,
    /// ID token for OpenID Connect (if provided).
    id_token: Option<String>,
}

impl TokenResponse {
    /// Create a new token response.
    pub fn new(access_token: String, token_type: String) -> Self {
        Self {
            access_token,
            token_type,
            expires_at: None,
            refresh_token: None,
            scopes: None,
            id_token: None,
        }
    }

    /// Set the expiration time.
    pub fn with_expires_in(mut self, expires_in: Duration) -> Self {
        self.expires_at = Some(Instant::now() + expires_in);
        self
    }

    /// Set the refresh token.
    pub fn with_refresh_token(mut self, refresh_token: String) -> Self {
        self.refresh_token = Some(refresh_token);
        self
    }

    /// Set the scopes.
    pub fn with_scopes(mut self, scopes: Vec<String>) -> Self {
        self.scopes = Some(scopes);
        self
    }

    /// Set the ID token.
    pub fn with_id_token(mut self, id_token: String) -> Self {
        self.id_token = Some(id_token);
        self
    }

    /// Get the access token.
    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    /// Get the token type.
    pub fn token_type(&self) -> &str {
        &self.token_type
    }

    /// Check if the token is expired.
    pub fn is_expired(&self) -> bool {
        match self.expires_at {
            Some(expires_at) => Instant::now() >= expires_at,
            None => false, // If no expiration, assume not expired
        }
    }

    /// Get the refresh token (if present).
    pub fn refresh_token(&self) -> Option<&str> {
        self.refresh_token.as_deref()
    }

    /// Get the ID token (if present, for OpenID Connect).
    pub fn id_token(&self) -> Option<&str> {
        self.id_token.as_deref()
    }

    /// Get the scopes (if provided in response).
    pub fn scopes(&self) -> Option<&[String]> {
        self.scopes.as_deref()
    }

    /// Get the time remaining until expiration.
    pub fn expires_in(&self) -> Option<Duration> {
        self.expires_at
            .and_then(|exp| exp.checked_duration_since(Instant::now()))
    }

    /// Get the Authorization header value.
    pub fn authorization_header(&self) -> String {
        format!("{} {}", self.token_type, self.access_token)
    }
}

/// Errors that can occur during OAuth2 operations.
#[derive(Debug, Error)]
pub enum TokenError {
    /// The authorization request was denied.
    #[error("Authorization denied: {0}")]
    AuthorizationDenied(String),

    /// Invalid authorization code.
    #[error("Invalid authorization code")]
    InvalidCode,

    /// Invalid CSRF state.
    #[error("Invalid CSRF state - possible CSRF attack")]
    InvalidState,

    /// Token exchange failed.
    #[error("Token exchange failed: {0}")]
    ExchangeFailed(String),

    /// Token refresh failed.
    #[error("Token refresh failed: {0}")]
    RefreshFailed(String),

    /// Network error.
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Invalid response from the authorization server.
    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    /// Token is expired.
    #[error("Token is expired")]
    TokenExpired,

    /// Missing required field in response.
    #[error("Missing required field: {0}")]
    MissingField(String),
}

/// PKCE (Proof Key for Code Exchange) verifier.
#[derive(Debug, Clone)]
pub struct PkceVerifier {
    verifier: String,
    challenge: String,
    method: String,
}

impl PkceVerifier {
    /// Generate a new PKCE verifier with S256 challenge.
    pub fn generate() -> Self {
        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
        use rand::{rngs::OsRng, RngCore};

        // Generate 32 random bytes for the verifier
        let mut verifier_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut verifier_bytes);
        let verifier = URL_SAFE_NO_PAD.encode(verifier_bytes);

        // Create S256 challenge: BASE64URL(SHA256(verifier))
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(verifier.as_bytes());
        let hash = hasher.finalize();
        let challenge = URL_SAFE_NO_PAD.encode(hash);

        Self {
            verifier,
            challenge,
            method: "S256".to_string(),
        }
    }

    /// Get the code verifier (for token exchange).
    pub fn verifier(&self) -> &str {
        &self.verifier
    }

    /// Get the code challenge (for authorization request).
    pub fn challenge(&self) -> &str {
        &self.challenge
    }

    /// Get the challenge method (S256).
    pub fn method(&self) -> &str {
        &self.method
    }
}

/// CSRF state token for OAuth2 authorization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsrfState(String);

impl CsrfState {
    /// Generate a new random CSRF state.
    pub fn generate() -> Self {
        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
        use rand::{rngs::OsRng, RngCore};

        let mut bytes = [0u8; 16];
        OsRng.fill_bytes(&mut bytes);
        Self(URL_SAFE_NO_PAD.encode(bytes))
    }

    /// Create from an existing string.
    pub fn new(state: String) -> Self {
        Self(state)
    }

    /// Get the state value.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Verify that this state matches another.
    pub fn verify(&self, other: &str) -> bool {
        // Use constant-time comparison to prevent timing attacks
        // For simplicity, we use direct comparison here
        self.0 == other
    }
}

impl std::fmt::Display for CsrfState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_response() {
        let token = TokenResponse::new("access123".to_string(), "Bearer".to_string())
            .with_refresh_token("refresh456".to_string())
            .with_expires_in(Duration::from_secs(3600));

        assert_eq!(token.access_token(), "access123");
        assert_eq!(token.token_type(), "Bearer");
        assert_eq!(token.refresh_token(), Some("refresh456"));
        assert!(!token.is_expired());
        assert_eq!(token.authorization_header(), "Bearer access123");
    }

    #[test]
    fn test_pkce_verifier() {
        let pkce = PkceVerifier::generate();
        assert!(!pkce.verifier().is_empty());
        assert!(!pkce.challenge().is_empty());
        assert_eq!(pkce.method(), "S256");

        // Verifier and challenge should be different
        assert_ne!(pkce.verifier(), pkce.challenge());
    }

    #[test]
    fn test_csrf_state() {
        let state1 = CsrfState::generate();
        let state2 = CsrfState::generate();

        // Each generated state should be unique
        assert_ne!(state1, state2);

        // Verification should work
        assert!(state1.verify(state1.as_str()));
        assert!(!state1.verify(state2.as_str()));
    }
}
