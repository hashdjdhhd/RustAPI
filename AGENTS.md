# AGENTS.md - RustAPI AI Agent Guide

> This file helps AI coding agents understand the RustAPI codebase quickly.

## Project Overview

**RustAPI** is a FastAPI-like web framework for Rust that combines Rust's performance and memory safety with FastAPI's developer experience.

**Core Philosophy:** "DX-First" - endpoint = 1 function + 1 attribute macro

**Goals:**
- Make Rust API development feel as easy as FastAPI
- Eliminate boilerplate: handler function + attribute macro = endpoint
- Automatic OpenAPI documentation (planned)
- Type-safe compile-time validation

**Status:** Phase 1 MVP Complete ✅ | Phase 2 (Validation + OpenAPI) In Progress 🔄

---

## Crate Structure

```
rustapi/
├── rustapi-rs/      # Public facade - published to crates.io
├── rustapi-core/    # Core implementation
├── rustapi-macros/  # Procedural macros
└── examples/
    └── hello-world/ # Example application
```

| Crate | Purpose |
|-------|---------|
| `rustapi-rs` | **Public facade** - the crate users depend on, re-exports core + macros |
| `rustapi-core` | **Core implementation** - router, extractors, handlers, server, errors |
| `rustapi-macros` | **Procedural macros** - `#[get]`, `#[post]`, etc. (planned) |

---

## Key Modules (rustapi-core)

| Module | Responsibility |
|--------|----------------|
| `app.rs` | `RustApi` builder - main entry point with `new()`, `route()`, `get()`, `post()`, `state()`, `run()` |
| `router.rs` | Radix tree router using `matchit`, method-based routing (GET, POST, PUT, DELETE, PATCH) |
| `handler.rs` | `Handler` trait with implementations for 0-5 argument handlers |
| `extract.rs` | Extractors: `Json<T>`, `Path<T>`, `Query<T>`, `State<T>`, `Body` |
| `response.rs` | `IntoResponse` trait, helpers: `json()`, `created()`, `no_content()`, `text()` |
| `error.rs` | `ApiError` struct, `Result<T>` type alias, JSON error format |
| `server.rs` | Hyper 1.x server with graceful shutdown (Ctrl+C) |

---

## Coding Conventions

### Import Pattern
```rust
use rustapi_rs::prelude::*;
```

### Handler Signatures
Handlers should be readable like documentation:
```rust
async fn get_user(Path(id): Path<u32>, State(db): State<Database>) -> Json<User> {
    // Business logic only
}
```

### Extractor Patterns
| Extractor | Usage | Route Example |
|-----------|-------|---------------|
| `Path<T>` | URL path parameters | `/users/{id}` |
| `Query<T>` | Query string params | `?page=1&limit=10` |
| `Json<T>` | JSON request body | POST/PUT requests |
| `State<T>` | Shared application state | Database connections |
| `Body` | Raw request body | File uploads |

### Response Patterns
```rust
Json(data)           // 200 OK with JSON
created(data)        // 201 Created with JSON
no_content()         // 204 No Content
text("hello")        // 200 OK with plain text
ApiError::not_found("User not found")  // 404 JSON error
```

### Path Parameter Syntax
- Use `{param}` format in routes (converted to `:param` internally)
- Example: `.get("/users/{id}", get_user)` extracts `id` parameter

### Wrapper Philosophy
**All dependencies are wrapped** - never exposed directly in public API:
- `matchit` → Router abstraction
- `hyper` → Server abstraction
- `validator` (planned) → Validation abstraction
- `utoipa` (planned) → Schema abstraction

---

## Standard JSON Error Format

```json
{
  "error": {
    "type": "validation_error",
    "message": "Request validation failed",
    "fields": [
      {"field": "email", "code": "email", "message": "Invalid email format"}
    ]
  }
}
```

---

## Build & Test Commands

```powershell
# Build the workspace
cargo build

# Run the hello-world example
cargo run -p hello-world

# Run all tests
cargo test --workspace

# Check for errors without building
cargo check --workspace

# Format code
cargo fmt --all

# Lint
cargo clippy --workspace
```

### Testing Endpoints (PowerShell)
```powershell
Invoke-RestMethod -Uri http://127.0.0.1:8080/
Invoke-RestMethod -Uri http://127.0.0.1:8080/health
Invoke-RestMethod -Uri http://127.0.0.1:8080/users/42
```

---

## Handler Trait System

```rust
pub trait Handler<T>: Clone + Send + Sync + 'static {
    fn call(self, req: Request) -> impl Future<Output = Response> + Send;
}
```

- Implemented for function pointers with **0-5 arguments**
- Each argument must implement `FromRequest` or `FromRequestParts`
- `FromRequestParts` - extractors that don't consume body (Path, Query, State)
- `FromRequest` - extractors that consume body (Json, Body)

---

## Development Phases

### ✅ Phase 1 - MVP (Complete)
- Workspace structure with 3 crates
- Hyper 1.x server with graceful shutdown
- Radix tree router (matchit)
- Handler trait (0-5 args)
- All core extractors: Json, Path, Query, State, Body
- IntoResponse + response helpers
- ApiError + Result type
- Tracing integration

### 🔄 Phase 2 - Validation + OpenAPI (Current)
- `rustapi-validate` crate - wrapper for `validator`
- `#[derive(Validate)]` macro
- Validation rules: email, length, range, regex, non_empty
- 422 JSON error format
- `rustapi-openapi` crate - wrapper for `utoipa`
- `/openapi.json` endpoint
- Swagger UI at `/docs`

### 📋 Phase 3-4 - Extras (Planned)
- Tower middleware integration
- JWT, CORS, rate limiting (`rustapi-extras`)
- Additional extractors: Headers, Cookies, IpAddr
- TestClient helper
- Benchmark suite

---

## Key Files to Read

| Purpose | File |
|---------|------|
| Example usage | `examples/hello-world/src/main.rs` |
| Core exports | `crates/rustapi-core/src/lib.rs` |
| App builder | `crates/rustapi-core/src/app.rs` |
| Router impl | `crates/rustapi-core/src/router.rs` |
| Extractors | `crates/rustapi-core/src/extract.rs` |
| Error handling | `crates/rustapi-core/src/error.rs` |
| Project context | `memories/rustapi_memory_bank.md` |
| Task list | `memories/TASKLIST.md` |

---

## Quick Reference

```rust
use rustapi_rs::prelude::*;

#[tokio::main]
async fn main() {
    RustApi::new()
        .get("/", index)
        .get("/users/{id}", get_user)
        .post("/users", create_user)
        .state(AppState::new())
        .run("127.0.0.1:8080")
        .await;
}

async fn index() -> &'static str {
    "Hello, RustAPI!"
}

async fn get_user(Path(id): Path<u32>) -> Json<User> {
    Json(User { id, name: "Alice".into() })
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    created(User { id: 1, name: payload.name })
}
```

---

## MSRV & License

- **Minimum Supported Rust Version:** 1.75+
- **License:** MIT OR Apache-2.0
