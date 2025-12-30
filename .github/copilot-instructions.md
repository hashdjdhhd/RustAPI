# RustAPI - Copilot Instructions

## Project Context
RustAPI is a FastAPI-like web framework for Rust. The core philosophy is **"DX-First"**: endpoint = 1 function + 1 attribute macro.

**Status:** Phase 1 MVP Complete | Phase 2 (Validation + OpenAPI) In Progress

## Architecture

**Workspace structure:**
- `rustapi-rs/` - Public facade crate (users depend on this)
- `rustapi-core/` - Core implementation (router, extractors, handlers, server)
- `rustapi-macros/` - Procedural macros (planned `#[get]`, `#[post]`)
- `examples/hello-world/` - Reference implementation

**Key design rule:** All dependencies are wrapped - never expose `hyper`, `matchit`, etc. directly.

## Code Patterns

### Always use prelude import
```rust
use rustapi_rs::prelude::*;
```

### Handler signature pattern
Handlers are async functions with up to 5 extractor arguments:
```rust
async fn get_user(Path(id): Path<u32>, State(db): State<Database>) -> Json<User> {
    // Business logic only - no framework ceremony
}
```

### Route registration
Use `{param}` syntax for path parameters (converted to `:param` internally):
```rust
RustApi::new()
    .route("/users/{id}", get(get_user))
    .route("/users", post(create_user))
```

### Response helpers
- `Json(data)` → 200 OK with JSON
- `created(data)` → 201 Created
- `no_content()` → 204 No Content
- `ApiError::not_found("msg")` → 404 JSON error

## Key Traits

| Trait | Purpose |
|-------|---------|
| `Handler<T>` | 0-5 arg async functions → endpoint handlers |
| `FromRequestParts` | Extractors that don't consume body (Path, Query, State) |
| `FromRequest` | Extractors that consume body (Json, Body) |
| `IntoResponse` | Types convertible to HTTP response |

## Commands

```powershell
cargo build                  # Build workspace
cargo run -p hello-world     # Run example server (http://127.0.0.1:8080)
cargo test --workspace       # Run all tests
cargo clippy --workspace     # Lint
```

## File Reference

| To understand... | Read |
|------------------|------|
| Usage example | `examples/hello-world/src/main.rs` |
| App builder | `crates/rustapi-core/src/app.rs` |
| Extractors | `crates/rustapi-core/src/extract.rs` |
| Handler trait | `crates/rustapi-core/src/handler.rs` |
| Error format | `crates/rustapi-core/src/error.rs` |
| Full context | `AGENTS.md`, `memories/rustapi_memory_bank.md` |
