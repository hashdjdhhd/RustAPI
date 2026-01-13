# Serverless Lambda Example

This example demonstrates how to deploy a Rust function to AWS Lambda using the AWS SAM CLI.

## Cold Start Optimization

Rust is an excellent choice for AWS Lambda due to its low memory footprint and fast startup times.

### 1. Minimal Binary Size
To minimize cold start times, we want the binary to be as small as possible.
Add this to your `Cargo.toml`:

```toml
[profile.release]
opt-level = "z"  # Optimize for size
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

### 2. Runtime
Use `provided.al2` or `provided.al2023` runtime (Custom Runtime) which has lower overhead than managed runtimes.

### 3. Initialization (Global State)
Perform heavy initialization (like parsing config, setting up DB clients, standard HTTP clients) **outside** the handler function (in `main` before `run`).
This allows AWS Lambda to reuse the execution environment for subsequent invocations ("warm starts"), skipping the initialization phase entirely.

### Build & Deploy

You can use `cargo-lambda` to build:

```bash
cargo lambda build --release
```

Or just SAM:

```bash
sam build
sam deploy --guided
```
