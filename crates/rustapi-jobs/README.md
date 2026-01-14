# RustAPI Jobs

**Background job processing for RustAPI.**

Offload heavy tasks (emails, report generation, webhooks) to background workers.

## Key Features

- **Backend Agnostic**: Drivers for **Redis** (recommended for speed) and **PostgreSQL** (for transactional reliability).
- **At-Least-Once Delivery**: Jobs are not lost if a worker crashes.
- **Retries**: Configurable exponential backoff policies.
- **Scheduling**: Cron-like recurring tasks.

## Quick Start

```rust
use rustapi_jobs::{Job, JobContext};

#[derive(Serialize, Deserialize)]
struct SendEmail {
    to: String,
    content: String,
}

#[async_trait]
impl Job for SendEmail {
    const NAME: &'static str = "send_email";

    async fn run(&self, _ctx: JobContext) -> Result<()> {
        // Send the email...
        Ok(())
    }
}

// Enqueue
queue.push(SendEmail { ... }).await?;
```
