//! # RustAPI
//!
//! A FastAPI-like web framework for Rust.
//!
//! RustAPI combines Rust's performance and safety with FastAPI's "just write business logic"
//! approach. It provides automatic OpenAPI documentation, declarative validation, and
//! a developer-friendly experience.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use rustapi_rs::prelude::*;
//!
//! #[derive(Serialize, Schema)]
//! struct Hello {
//!     message: String,
//! }
//!
//! async fn hello() -> Json<Hello> {
//!     Json(Hello {
//!         message: "Hello, World!".to_string(),
//!     })
//! }
//!
//! #[tokio::main]
//! async fn main() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     RustApi::new()
//!         .route("/", get(hello))
//!         .run("127.0.0.1:8080")
//!         .await
//! }
//! ```
//!
//! ## Features
//!
//! - **DX-First**: Minimal boilerplate, intuitive API
//! - **Type-Safe**: Compile-time route and schema validation
//! - **Auto Documentation**: OpenAPI + Swagger UI out of the box
//! - **Declarative Validation**: Pydantic-style validation on structs
//! - **Batteries Included**: JWT, CORS, rate limiting (optional features)
//!
//! ## Optional Features
//!
//! RustAPI provides several optional features that can be enabled via Cargo feature flags:
//!
//! - `jwt` - JWT authentication middleware and `AuthUser<T>` extractor
//! - `cors` - CORS middleware with builder pattern configuration
//! - `rate-limit` - IP-based rate limiting middleware
//! - `config` - Configuration management with `.env` file support
//! - `cookies` - Cookie parsing extractor
//! - `sqlx` - SQLx database error conversion to ApiError
//! - `extras` - Meta feature enabling jwt, cors, and rate-limit
//! - `full` - All optional features enabled
//!
//! ### Example with JWT and CORS
//!
//! ```toml
//! [dependencies]
//! rustapi-rs = { version = "0.1", features = ["jwt", "cors"] }
//! ```
//!
//! ```rust,ignore
//! use rustapi_rs::prelude::*;
//! use rustapi_rs::jwt::{JwtLayer, AuthUser};
//! use rustapi_rs::cors::CorsLayer;
//!
//! #[derive(Deserialize)]
//! struct Claims {
//!     sub: String,
//!     exp: u64,
//! }
//!
//! async fn protected(AuthUser(claims): AuthUser<Claims>) -> String {
//!     format!("Hello, {}!", claims.sub)
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     RustApi::new()
//!         .layer(CorsLayer::permissive())
//!         .layer(JwtLayer::<Claims>::new("secret"))
//!         .route("/protected", get(protected))
//!         .run("127.0.0.1:8080")
//!         .await
//! }
//! ```

// Re-export core functionality
pub use rustapi_core::*;

// Re-export macros
pub use rustapi_macros::*;

// Re-export rustapi-extras modules conditionally based on features
#[cfg(feature = "jwt")]
pub mod jwt {
    //! JWT authentication middleware and extractors.
    //!
    //! This module provides JWT token validation middleware and the `AuthUser<T>` extractor
    //! for accessing decoded token claims in handlers.
    //!
    //! # Example
    //!
    //! ```rust,ignore
    //! use rustapi_rs::prelude::*;
    //! use rustapi_rs::jwt::{JwtLayer, AuthUser, create_token};
    //!
    //! #[derive(Serialize, Deserialize)]
    //! struct Claims {
    //!     sub: String,
    //!     exp: u64,
    //! }
    //!
    //! async fn protected(AuthUser(claims): AuthUser<Claims>) -> String {
    //!     format!("Hello, {}!", claims.sub)
    //! }
    //! ```
    pub use rustapi_extras::jwt::*;
}

#[cfg(feature = "cors")]
pub mod cors {
    //! CORS (Cross-Origin Resource Sharing) middleware.
    //!
    //! This module provides configurable CORS middleware with a builder pattern API.
    //!
    //! # Example
    //!
    //! ```rust,ignore
    //! use rustapi_rs::prelude::*;
    //! use rustapi_rs::cors::CorsLayer;
    //!
    //! RustApi::new()
    //!     .layer(CorsLayer::permissive())
    //!     // or configure specific origins:
    //!     // .layer(CorsLayer::new().allow_origins(["https://example.com"]))
    //!     .route("/", get(handler))
    //!     .run("127.0.0.1:8080")
    //!     .await
    //! ```
    pub use rustapi_extras::cors::*;
}

#[cfg(feature = "rate-limit")]
pub mod rate_limit {
    //! IP-based rate limiting middleware.
    //!
    //! This module provides rate limiting middleware that tracks requests per client IP.
    //!
    //! # Example
    //!
    //! ```rust,ignore
    //! use rustapi_rs::prelude::*;
    //! use rustapi_rs::rate_limit::RateLimitLayer;
    //! use std::time::Duration;
    //!
    //! RustApi::new()
    //!     .layer(RateLimitLayer::new(100, Duration::from_secs(60))) // 100 requests per minute
    //!     .route("/", get(handler))
    //!     .run("127.0.0.1:8080")
    //!     .await
    //! ```
    pub use rustapi_extras::rate_limit::*;
}

#[cfg(feature = "config")]
pub mod config {
    //! Configuration management with `.env` file support.
    //!
    //! This module provides utilities for loading configuration from environment variables
    //! and `.env` files.
    //!
    //! # Example
    //!
    //! ```rust,ignore
    //! use rustapi_rs::prelude::*;
    //! use rustapi_rs::config::{Config, load_dotenv, Environment};
    //!
    //! #[derive(Deserialize)]
    //! struct AppConfig {
    //!     database_url: String,
    //!     port: u16,
    //! }
    //!
    //! #[tokio::main]
    //! async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //!     load_dotenv();
    //!     
    //!     let env = Environment::current();
    //!     println!("Running in {:?} mode", env);
    //!     
    //!     // Config can be extracted in handlers
    //!     RustApi::new()
    //!         .route("/", get(handler))
    //!         .run("127.0.0.1:8080")
    //!         .await
    //! }
    //! ```
    pub use rustapi_extras::config::*;
}

#[cfg(feature = "sqlx")]
pub mod sqlx {
    //! SQLx database integration utilities.
    //!
    //! This module provides error conversion from SQLx errors to ApiError responses.
    //!
    //! # Example
    //!
    //! ```rust,ignore
    //! use rustapi_rs::prelude::*;
    //! use rustapi_rs::sqlx::SqlxErrorExt;
    //!
    //! async fn get_user(State(pool): State<PgPool>, Path(id): Path<i64>) -> Result<Json<User>> {
    //!     let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
    //!         .fetch_one(&pool)
    //!         .await
    //!         .to_api_error()?;
    //!     Ok(Json(user))
    //! }
    //! ```
    pub use rustapi_extras::sqlx::*;
}

/// Prelude module - import everything you need with `use rustapi_rs::prelude::*`
pub mod prelude {
    // Core types
    pub use rustapi_core::{
        // App builder
        RustApi,
        // Router
        Router,
        get, post, put, patch, delete,
        // Route type for macro-based routing
        Route,
        get_route, post_route, put_route, patch_route, delete_route,
        // Extractors
        Json, Query, Path, State, Body,
        ValidatedJson,
        Headers, HeaderValue, Extension, ClientIp,
        // Response types
        IntoResponse, Response,
        Created, NoContent, Html, Redirect, WithStatus,
        // Streaming responses
        Sse, SseEvent, StreamBody,
        // Middleware
        RequestId, RequestIdLayer, TracingLayer,
        // Error handling
        ApiError, Result,
        // Request context
        Request,
    };

    // Cookies extractor (feature-gated)
    #[cfg(feature = "cookies")]
    pub use rustapi_core::Cookies;

    // Re-export the route! macro
    pub use rustapi_core::route;

    // Re-export validation - use validator derive macro directly
    pub use validator::Validate;
    
    // Re-export OpenAPI schema derive
    pub use rustapi_openapi::{Schema, IntoParams};

    // Re-export commonly used external types
    pub use serde::{Deserialize, Serialize};
    pub use tracing::{debug, error, info, trace, warn};

    // JWT types (feature-gated)
    #[cfg(feature = "jwt")]
    pub use rustapi_extras::jwt::{AuthUser, JwtLayer, JwtValidation, create_token};

    // CORS types (feature-gated)
    #[cfg(feature = "cors")]
    pub use rustapi_extras::cors::{CorsLayer, AllowedOrigins};

    // Rate limiting types (feature-gated)
    #[cfg(feature = "rate-limit")]
    pub use rustapi_extras::rate_limit::RateLimitLayer;

    // Config types (feature-gated)
    #[cfg(feature = "config")]
    pub use rustapi_extras::config::{Config, Environment, load_dotenv, env_or, require_env};
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn prelude_imports_work() {
        // This test ensures prelude exports compile correctly
        let _: fn() -> Result<()> = || Ok(());
    }
}
