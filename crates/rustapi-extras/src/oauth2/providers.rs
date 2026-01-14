//! OAuth2 provider presets
//!
//! Pre-configured settings for common OAuth2 providers.

use std::collections::HashSet;

/// Supported OAuth2 providers with pre-configured endpoints.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Provider {
    /// Google OAuth2
    Google,
    /// GitHub OAuth2
    GitHub,
    /// Microsoft (Azure AD) OAuth2
    Microsoft,
    /// Discord OAuth2
    Discord,
    /// Custom provider with manual configuration
    Custom {
        /// Authorization endpoint URL
        auth_url: String,
        /// Token endpoint URL
        token_url: String,
        /// User info endpoint URL (optional)
        userinfo_url: Option<String>,
    },
}

impl Provider {
    /// Get the authorization endpoint URL for this provider.
    pub fn auth_url(&self) -> &str {
        match self {
            Provider::Google => "https://accounts.google.com/o/oauth2/v2/auth",
            Provider::GitHub => "https://github.com/login/oauth/authorize",
            Provider::Microsoft => "https://login.microsoftonline.com/common/oauth2/v2.0/authorize",
            Provider::Discord => "https://discord.com/api/oauth2/authorize",
            Provider::Custom { auth_url, .. } => auth_url,
        }
    }

    /// Get the token endpoint URL for this provider.
    pub fn token_url(&self) -> &str {
        match self {
            Provider::Google => "https://oauth2.googleapis.com/token",
            Provider::GitHub => "https://github.com/login/oauth/access_token",
            Provider::Microsoft => "https://login.microsoftonline.com/common/oauth2/v2.0/token",
            Provider::Discord => "https://discord.com/api/oauth2/token",
            Provider::Custom { token_url, .. } => token_url,
        }
    }

    /// Get the user info endpoint URL for this provider (if available).
    pub fn userinfo_url(&self) -> Option<&str> {
        match self {
            Provider::Google => Some("https://www.googleapis.com/oauth2/v3/userinfo"),
            Provider::GitHub => Some("https://api.github.com/user"),
            Provider::Microsoft => Some("https://graph.microsoft.com/v1.0/me"),
            Provider::Discord => Some("https://discord.com/api/users/@me"),
            Provider::Custom { userinfo_url, .. } => userinfo_url.as_deref(),
        }
    }

    /// Get default scopes for this provider.
    pub fn default_scopes(&self) -> HashSet<String> {
        match self {
            Provider::Google => ["openid", "email", "profile"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            Provider::GitHub => ["user:email", "read:user"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            Provider::Microsoft => ["openid", "email", "profile", "User.Read"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            Provider::Discord => ["identify", "email"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            Provider::Custom { .. } => HashSet::new(),
        }
    }

    /// Check if this provider supports PKCE (Proof Key for Code Exchange).
    pub fn supports_pkce(&self) -> bool {
        match self {
            Provider::Google => true,
            Provider::GitHub => false, // GitHub doesn't support PKCE yet
            Provider::Microsoft => true,
            Provider::Discord => true,
            Provider::Custom { .. } => true, // Assume custom supports PKCE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_provider() {
        let provider = Provider::Google;
        assert!(provider.auth_url().contains("google.com"));
        assert!(provider.token_url().contains("googleapis.com"));
        assert!(provider.supports_pkce());
        assert!(provider.default_scopes().contains("openid"));
    }

    #[test]
    fn test_github_provider() {
        let provider = Provider::GitHub;
        assert!(provider.auth_url().contains("github.com"));
        assert!(!provider.supports_pkce());
        assert!(provider.default_scopes().contains("user:email"));
    }

    #[test]
    fn test_custom_provider() {
        let provider = Provider::Custom {
            auth_url: "https://custom.example.com/auth".to_string(),
            token_url: "https://custom.example.com/token".to_string(),
            userinfo_url: Some("https://custom.example.com/userinfo".to_string()),
        };
        assert_eq!(provider.auth_url(), "https://custom.example.com/auth");
        assert_eq!(provider.token_url(), "https://custom.example.com/token");
        assert_eq!(
            provider.userinfo_url(),
            Some("https://custom.example.com/userinfo")
        );
    }
}
