# RustAPI Macros

**Procedural macros that power the RustAPI developer experience.**

> ℹ️ **Note**: These macros are re-exported by `rustapi-rs`. You do not need to add this crate manually.

## Attribute Macros

### `#[rustapi::main]`
Replaces `#[tokio::main]`. Sets up the runtime, tracing subscriber, and other framework essentials.

### HTTP Method Handlers
Registers a function as a route handler.

- `#[rustapi::get("/users/{id}")]`
- `#[rustapi::post("/users")]`
- `#[rustapi::put("/users/{id}")]`
- `#[rustapi::delete("/users/{id}")]`
- `#[rustapi::patch("/users/{id}")]`
- `#[rustapi::head("/health")]`
- `#[rustapi::options("/cors")]`

### OpenAPI Metadata
Enrich your auto-generated documentation.

- `#[rustapi::tag("Auth")]`: Groups endpoints.
- `#[rustapi::summary("Logs in a user")]`: Brief summary.
- `#[rustapi::description("Full markdown description...")]`: Detailed docs.

## Derive Macros

### `#[derive(Schema)]`
Generates a JSON Schema for the struct, used by `rustapi-openapi`.
*Wraps `utoipa::ToSchema` via `rustapi-openapi` integration.*

### `#[derive(Validate)]`
Generates validation logic.
*Wraps `validator::Validate` via `rustapi-validate` integration.*
