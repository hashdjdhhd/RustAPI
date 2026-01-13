//! OAuth2 configuration

use super::providers::Provider;
use std::collections::HashSet;
use std::time::Duration;

/// Configuration for OAuth2 authentication.
#[derive(Debug, Clone)]
pub struct OAuth2Config {
    /// The OAuth2 provider (includes endpoint URLs).
    pub(crate) provider: Provider,
    /// Client ID issued by the provider.
    pub(crate) client_id: String,
    /// Client secret issued by the provider.
    pub(crate) client_secret: String,
    /// Redirect URI for the authorization callback.
    pub(crate) redirect_uri: String,
    /// Scopes to request.
    pub(crate) scopes: HashSet<String>,
    /// Whether to use PKCE (Proof Key for Code Exchange).
    pub(crate) use_pkce: bool,
    /// Timeout for HTTP requests.
    pub(crate) timeout: Duration,
}

impl OAuth2Config {
    /// Create a new OAuth2 configuration with a custom provider.
    pub fn new(
        provider: Provider,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        redirect_uri: impl Into<String>,
    ) -> Self {
        let provider_clone = provider.clone();
        Self {
            scopes: provider.default_scopes(),
            use_pkce: provider.supports_pkce(),
            provider: provider_clone,
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            redirect_uri: redirect_uri.into(),
            timeout: Duration::from_secs(30),
        }
    }

    /// Create a Google OAuth2 configuration.
    pub fn google(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        redirect_uri: impl Into<String>,
    ) -> Self {
        Self::new(Provider::Google, client_id, client_secret, redirect_uri)
    }

    /// Create a GitHub OAuth2 configuration.
    pub fn github(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        redirect_uri: impl Into<String>,
    ) -> Self {
        Self::new(Provider::GitHub, client_id, client_secret, redirect_uri)
    }

    /// Create a Microsoft OAuth2 configuration.
    pub fn microsoft(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        redirect_uri: impl Into<String>,
    ) -> Self {
        Self::new(Provider::Microsoft, client_id, client_secret, redirect_uri)
    }

    /// Create a Discord OAuth2 configuration.
    pub fn discord(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        redirect_uri: impl Into<String>,
    ) -> Self {
        Self::new(Provider::Discord, client_id, client_secret, redirect_uri)
    }

    /// Create a custom OAuth2 configuration.
    pub fn custom(
        auth_url: impl Into<String>,
        token_url: impl Into<String>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        redirect_uri: impl Into<String>,
    ) -> Self {
        Self::new(
            Provider::Custom {
                auth_url: auth_url.into(),
                token_url: token_url.into(),
                userinfo_url: None,
            },
            client_id,
            client_secret,
            redirect_uri,
        )
    }

    /// Add a scope to request.
    pub fn scope(mut self, scope: impl Into<String>) -> Self {
        self.scopes.insert(scope.into());
        self
    }

    /// Set multiple scopes (replaces existing).
    pub fn scopes<I, S>(mut self, scopes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.scopes = scopes.into_iter().map(Into::into).collect();
        self
    }

    /// Enable or disable PKCE.
    pub fn pkce(mut self, enabled: bool) -> Self {
        self.use_pkce = enabled;
        self
    }

    /// Set the HTTP request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Get the client ID.
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Get the redirect URI.
    pub fn redirect_uri(&self) -> &str {
        &self.redirect_uri
    }

    /// Get the provider.
    pub fn provider(&self) -> &Provider {
        &self.provider
    }

    /// Get the scopes.
    pub fn get_scopes(&self) -> &HashSet<String> {
        &self.scopes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_config() {
        let config = OAuth2Config::google("id", "secret", "https://example.com/callback");
        assert_eq!(config.client_id(), "id");
        assert!(config.use_pkce);
        assert!(config.scopes.contains("openid"));
    }

    #[test]
    fn test_scope_builder() {
        let config = OAuth2Config::github("id", "secret", "https://example.com/callback")
            .scope("repo")
            .scope("gist");

        assert!(config.scopes.contains("repo"));
        assert!(config.scopes.contains("gist"));
        assert!(config.scopes.contains("user:email")); // Default scope still present
    }

    #[test]
    fn test_custom_provider() {
        let config = OAuth2Config::custom(
            "https://auth.example.com/authorize",
            "https://auth.example.com/token",
            "my_client",
            "my_secret",
            "https://myapp.com/callback",
        );

        assert_eq!(
            config.provider.auth_url(),
            "https://auth.example.com/authorize"
        );
        assert_eq!(
            config.provider.token_url(),
            "https://auth.example.com/token"
        );
    }
}
