//! OAuth2 client implementation

use super::config::OAuth2Config;
use super::tokens::{CsrfState, PkceVerifier, TokenError, TokenResponse};
use std::collections::HashMap;
use std::time::Duration;

/// OAuth2 client for handling authorization flows.
#[derive(Debug, Clone)]
pub struct OAuth2Client {
    config: OAuth2Config,
}

impl OAuth2Client {
    /// Create a new OAuth2 client.
    pub fn new(config: OAuth2Config) -> Self {
        Self { config }
    }

    /// Get the configuration.
    pub fn config(&self) -> &OAuth2Config {
        &self.config
    }

    /// Generate an authorization URL for the user to visit.
    ///
    /// Returns the authorization URL, CSRF state token, and optionally a PKCE verifier.
    pub fn authorization_url(&self) -> AuthorizationRequest {
        let csrf_state = CsrfState::generate();
        let pkce = if self.config.use_pkce {
            Some(PkceVerifier::generate())
        } else {
            None
        };

        // Build query parameters
        let mut params = vec![
            ("client_id", self.config.client_id.clone()),
            ("redirect_uri", self.config.redirect_uri.clone()),
            ("response_type", "code".to_string()),
            ("state", csrf_state.as_str().to_string()),
        ];

        // Add scopes
        if !self.config.scopes.is_empty() {
            let scope_str = self
                .config
                .scopes
                .iter()
                .cloned()
                .collect::<Vec<_>>()
                .join(" ");
            params.push(("scope", scope_str));
        }

        // Add PKCE parameters if enabled
        if let Some(ref pkce) = pkce {
            params.push(("code_challenge", pkce.challenge().to_string()));
            params.push(("code_challenge_method", pkce.method().to_string()));
        }

        // Build the URL
        let query = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!("{}?{}", self.config.provider.auth_url(), query);

        AuthorizationRequest {
            url,
            csrf_state,
            pkce_verifier: pkce,
        }
    }

    /// Exchange an authorization code for tokens.
    ///
    /// This should be called after the user is redirected back with the authorization code.
    pub async fn exchange_code(
        &self,
        code: &str,
        pkce_verifier: Option<&PkceVerifier>,
    ) -> Result<TokenResponse, TokenError> {
        let mut params = HashMap::new();
        params.insert("grant_type", "authorization_code".to_string());
        params.insert("code", code.to_string());
        params.insert("client_id", self.config.client_id.clone());
        params.insert("client_secret", self.config.client_secret.clone());
        params.insert("redirect_uri", self.config.redirect_uri.clone());

        // Add PKCE verifier if provided
        if let Some(verifier) = pkce_verifier {
            params.insert("code_verifier", verifier.verifier().to_string());
        }

        self.token_request(params).await
    }

    /// Refresh an access token using a refresh token.
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<TokenResponse, TokenError> {
        let mut params = HashMap::new();
        params.insert("grant_type", "refresh_token".to_string());
        params.insert("refresh_token", refresh_token.to_string());
        params.insert("client_id", self.config.client_id.clone());
        params.insert("client_secret", self.config.client_secret.clone());

        self.token_request(params).await
    }

    /// Make a token request to the authorization server.
    async fn token_request(
        &self,
        params: HashMap<&str, String>,
    ) -> Result<TokenResponse, TokenError> {
        // Build form data
        let form_data = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        // Make HTTP request
        let client = reqwest::Client::builder()
            .timeout(self.config.timeout)
            .build()
            .map_err(|e| TokenError::NetworkError(e.to_string()))?;

        let response = client
            .post(self.config.provider.token_url())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Accept", "application/json")
            .body(form_data)
            .send()
            .await
            .map_err(|e| TokenError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(TokenError::ExchangeFailed(error_text));
        }

        // Parse response
        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| TokenError::InvalidResponse(e.to_string()))?;

        self.parse_token_response(response_json)
    }

    /// Parse a token response from JSON.
    fn parse_token_response(&self, json: serde_json::Value) -> Result<TokenResponse, TokenError> {
        let access_token = json
            .get("access_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| TokenError::MissingField("access_token".to_string()))?
            .to_string();

        let token_type = json
            .get("token_type")
            .and_then(|v| v.as_str())
            .unwrap_or("Bearer")
            .to_string();

        let mut response = TokenResponse::new(access_token, token_type);

        // Optional fields
        if let Some(expires_in) = json.get("expires_in").and_then(|v| v.as_u64()) {
            response = response.with_expires_in(Duration::from_secs(expires_in));
        }

        if let Some(refresh) = json.get("refresh_token").and_then(|v| v.as_str()) {
            response = response.with_refresh_token(refresh.to_string());
        }

        if let Some(id_token) = json.get("id_token").and_then(|v| v.as_str()) {
            response = response.with_id_token(id_token.to_string());
        }

        if let Some(scope) = json.get("scope").and_then(|v| v.as_str()) {
            let scopes: Vec<String> = scope.split(' ').map(String::from).collect();
            response = response.with_scopes(scopes);
        }

        Ok(response)
    }

    /// Validate the CSRF state from the callback.
    pub fn validate_state(&self, expected: &CsrfState, received: &str) -> Result<(), TokenError> {
        if expected.verify(received) {
            Ok(())
        } else {
            Err(TokenError::InvalidState)
        }
    }
}

/// Authorization request containing the URL and security tokens.
#[derive(Debug)]
pub struct AuthorizationRequest {
    /// The authorization URL to redirect the user to.
    pub url: String,
    /// CSRF state token (store this to verify callback).
    pub csrf_state: CsrfState,
    /// PKCE verifier (store this for token exchange, if PKCE is enabled).
    pub pkce_verifier: Option<PkceVerifier>,
}

impl AuthorizationRequest {
    /// Get just the authorization URL.
    pub fn url(&self) -> &str {
        &self.url
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::oauth2::OAuth2Config;

    #[test]
    fn test_authorization_url_google() {
        let config = OAuth2Config::google(
            "test_client_id",
            "test_client_secret",
            "https://example.com/callback",
        );
        let client = OAuth2Client::new(config);
        let auth_req = client.authorization_url();

        // Check URL structure
        assert!(auth_req.url.contains("accounts.google.com"));
        assert!(auth_req.url.contains("client_id=test_client_id"));
        assert!(auth_req.url.contains("redirect_uri="));
        assert!(auth_req.url.contains("response_type=code"));
        assert!(auth_req.url.contains("state="));
        assert!(auth_req.url.contains("code_challenge=")); // PKCE enabled for Google

        // Check CSRF state is generated
        assert!(!auth_req.csrf_state.as_str().is_empty());

        // Check PKCE verifier is generated (Google supports PKCE)
        assert!(auth_req.pkce_verifier.is_some());
    }

    #[test]
    fn test_authorization_url_github() {
        let config = OAuth2Config::github(
            "test_client_id",
            "test_client_secret",
            "https://example.com/callback",
        );
        let client = OAuth2Client::new(config);
        let auth_req = client.authorization_url();

        // Check URL structure
        assert!(auth_req.url.contains("github.com"));
        assert!(auth_req.url.contains("client_id=test_client_id"));

        // GitHub doesn't support PKCE
        assert!(auth_req.pkce_verifier.is_none());
        assert!(!auth_req.url.contains("code_challenge="));
    }

    #[test]
    fn test_state_validation() {
        let config = OAuth2Config::google("id", "secret", "https://example.com/callback");
        let client = OAuth2Client::new(config);

        let state = CsrfState::generate();

        // Valid state should pass
        assert!(client.validate_state(&state, state.as_str()).is_ok());

        // Invalid state should fail
        assert!(matches!(
            client.validate_state(&state, "wrong_state"),
            Err(TokenError::InvalidState)
        ));
    }

    #[test]
    fn test_parse_token_response() {
        let config = OAuth2Config::google("id", "secret", "https://example.com/callback");
        let client = OAuth2Client::new(config);

        let json = serde_json::json!({
            "access_token": "ya29.access_token_here",
            "token_type": "Bearer",
            "expires_in": 3600,
            "refresh_token": "1//refresh_token_here",
            "scope": "openid email profile",
            "id_token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9..."
        });

        let result = client.parse_token_response(json);
        assert!(result.is_ok());

        let token = result.unwrap();
        assert_eq!(token.access_token(), "ya29.access_token_here");
        assert_eq!(token.token_type(), "Bearer");
        assert_eq!(token.refresh_token(), Some("1//refresh_token_here"));
        assert!(token.id_token().is_some());
        assert!(!token.is_expired());
    }
}
