# RustAPI Jobs

Robust background job processing for the RustAPI framework.

## Features

- **Multiple Backends**: Support for Redis and PostgreSQL backends.
- **Reliable Processing**: At-least-once delivery guarantees.
- **Retries**: Configurable retry policies with exponential backoff.
- **Scheduled Tasks**: Cron-like scheduling for recurring jobs.
- **Async Processing**: Fully async/await based on Tokio.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
rustapi-jobs = { version = "0.1", features = ["redis"] }
```

### Defining a Job

```rust
use rustapi_jobs::{Job, JobContext};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct EmailJob {
    to: String,
    subject: String,
    body: String,
}

#[async_trait]
impl Job for EmailJob {
    const NAME: &'static str = "send_email";

    async fn run(&self, _ctx: JobContext) -> Result<(), Error> {
        // Send email...
        Ok(())
    }
}
```

### Enqueueing Jobs

```rust
let queue = RedisQueue::new("redis://localhost:6379").await?;
queue.push(EmailJob {
    to: "user@example.com".to_string(),
    subject: "Welcome!".to_string(),
    body: "Hello...".to_string(),
}).await?;
```
