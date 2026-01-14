# RustAPI Core

**The internal engine of the RustAPI framework.**

> ⚠️ **Warning**: Most users should depend on `rustapi-rs` instead of using this crate directly. This crate's API is subject to change to support the higher-level facade.

## Overview

`rustapi-core` handles the low-level HTTP mechanics, leveraging the best-in-class Rust async ecosystem:

- **Hyper 1.0**: For robust, correct HTTP/1 and HTTP/2 implementation.
- **Matchit**: For extremely fast URL routing (radix tree based).
- **Tower**: For middleware composition (Service, Layer).
- **Tokio**: For the async runtime.

## Key Concepts

### The `RustApi` Builder
Responsible for assembling routes, middleware, and starting the Hyper server.

### The `Handler` Trait
The magic that allows functions with arbitrary arguments (extractors) to be used as request handlers.

```rust
// This function...
async fn my_handler(Json(body): Json<MyData>) -> impl Responder { ... }

// ...is converted into a Tower Service via the Handler trait.
```

### Extractors
`FromRequest` and `FromRequestParts` traits are defined here. They allow type-safe extraction of data from HTTP requests.

- `Json<T>`
- `Query<T>`
- `Path<T>`
- `State<T>`
