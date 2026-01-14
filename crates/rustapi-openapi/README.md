# RustAPI OpenAPI

**Automated API specifications and Swagger UI integration.**

> ℹ️ **Note**: This crate is used internally by `rustapi-rs` to provide the `.docs()` method on the server builder.

## How It Works

1.  **Reflection**: RustAPI macros collect metadata about your routes (path, method, input types, output types) at compile time.
2.  **Schema Gen**: It uses `utoipa` to generate JSON Schemas for your Rust structs.
3.  **Spec Build**: At runtime, it assembles the full OpenAPI 3.0 JSON specification.
4.  **UI Serve**: It embeds the Swagger UI assets and serves them at your specified path.

## Customization

You can inject custom security schemes or info into the spec via the `RustApi` builder.

```rust
RustApi::new()
    .api_name("My Enterprise API")
    .api_version("2.1.0")
    .docs("/swagger-ui")
    .run("0.0.0.0:3000")
    .await
```
