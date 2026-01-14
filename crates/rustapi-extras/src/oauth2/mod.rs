//! OAuth2 client integration for RustAPI
//!
//! This module provides OAuth2 authentication support with built-in
//! provider presets for common identity providers.
//!
//! # Example
//!
//! ```rust,no_run
//! use rustapi_extras::oauth2::{OAuth2Client, OAuth2Config, Provider};
//!
//! // Using a preset provider
//! let config = OAuth2Config::google(
//!     "client_id",
//!     "client_secret",
//!     "https://myapp.com/auth/callback",
//! );
//!
//! let client = OAuth2Client::new(config);
//!
//! // Generate authorization URL
//! let auth_request = client.authorization_url();
//! let auth_url = auth_request.url();
//! let csrf_state = &auth_request.csrf_state;
//! let pkce_verifier = &auth_request.pkce_verifier;
//!
//! // After user authorization, exchange the code
//! // let tokens = client.exchange_code("auth_code", pkce_verifier.as_ref()).await?;
//! ```

mod client;
mod config;
mod providers;
mod tokens;

pub use client::{AuthorizationRequest, OAuth2Client};
pub use config::OAuth2Config;
pub use providers::Provider;
pub use tokens::{CsrfState, PkceVerifier, TokenError, TokenResponse};
