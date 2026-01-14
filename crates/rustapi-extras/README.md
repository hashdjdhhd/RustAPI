# RustAPI Extras

**Production-ready middleware and utilities for RustAPI.**

This crate provides optional "batteries" that you can enable to build robust applications.

## Feature Flags

Enable these in your `Cargo.toml`.

| Feature | Description | Dependencies |
|---------|-------------|--------------|
| `jwt` | JSON Web Token authentication extractor & middleware | `jsonwebtoken` |
| `cors` | Cross-Origin Resource Sharing middleware | `tower-http` |
| `rate-limit` | IP-based rate limiting | `governor` / `dashmap` |
| `sqlx` | Database integration helpers | `sqlx` |
| `config` | Typed configuration loading from env/files | `config`, `dotenvy` |
| `otel` | OpenTelemetry observability integration | `opentelemetry` |

## Usage Examples

### JWT Authentication

```rust
use rustapi_rs::prelude::*;
use rustapi_extras::jwt::{JwtAuth, AuthUser};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[get("/protected")]
async fn protected_route(auth: AuthUser<Claims>) -> impl Responder {
    format!("Hello user {}", auth.sub)
}
```

### CORS

```rust
use rustapi_extras::cors::CorsLayer;

RustApi::new()
    .layer(CorsLayer::permissive())
    // ...
```
