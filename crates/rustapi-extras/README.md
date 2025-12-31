# rustapi-extras

Optional security and utility features for the RustAPI framework.

## Features

This crate provides production-ready middleware and utilities that are opt-in via Cargo feature flags to minimize binary size when not needed.

### Available Features

- `jwt` - JWT authentication middleware and `AuthUser<T>` extractor
- `cors` - CORS middleware with builder pattern configuration
- `rate-limit` - IP-based rate limiting middleware
- `config` - Configuration management with `.env` file support
- `cookies` - Cookie parsing extractor
- `extras` - Meta feature enabling jwt, cors, and rate-limit
- `full` - All features enabled

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
rustapi-extras = { version = "0.1", features = ["jwt", "cors"] }
```

## Examples

### JWT Authentication

```rust
use rustapi_extras::jwt::{JwtLayer, AuthUser};
use serde::Deserialize;

#[derive(Deserialize)]
struct Claims {
    sub: String,
    exp: u64,
}

async fn protected(AuthUser(claims): AuthUser<Claims>) -> String {
    format!("Hello, {}", claims.sub)
}
```

### CORS Configuration

```rust
use rustapi_extras::cors::CorsLayer;
use http::Method;

let cors = CorsLayer::new()
    .allow_origins(["https://example.com"])
    .allow_methods([Method::GET, Method::POST])
    .allow_credentials(true);
```

### Rate Limiting

```rust
use rustapi_extras::rate_limit::RateLimitLayer;
use std::time::Duration;

// Allow 100 requests per minute per IP
let rate_limit = RateLimitLayer::new(100, Duration::from_secs(60));
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
