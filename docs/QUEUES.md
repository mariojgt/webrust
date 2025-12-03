# ⏳ Job Queues

WebRust provides a unified API across a variety of different queue backends. Queues allow you to defer the processing of a time-consuming task, such as sending an email, until a later time. This drastically speeds up web requests to your application.

## Configuration

The queue configuration is located in `src/config/queue.rs`.

Supported drivers:
- `sync` (default): Jobs are executed immediately within the request lifecycle (blocking or spawned).
- `redis`: Jobs are sent to a Redis list to be processed by a worker.

To use Redis, update your `.env` file (or `src/config/queue.rs` defaults):

```rust
pub struct QueueConfig {
    pub driver: "redis".to_string(),
    pub redis_url: "redis://127.0.0.1:6379/".to_string(),
    pub queue_name: "default".to_string(),
}
```

## Creating Jobs

A job is a struct that implements the `Job` trait. It must be serializable.

```rust
use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use crate::services::queue::Job;

#[derive(Serialize, Deserialize, Debug)]
pub struct SendWelcomeEmail {
    pub user_id: i64,
}

#[async_trait]
impl Job for SendWelcomeEmail {
    fn name(&self) -> String {
        "SendWelcomeEmail".to_string()
    }

    async fn handle(&self) -> Result<(), String> {
        println!("Processing SendWelcomeEmail for user {}", self.user_id);
        // Logic to send email...
        Ok(())
    }
}
```

## Dispatching Jobs

You can dispatch jobs from anywhere in your application (Controllers, Routes, etc.):

```rust
use crate::services::queue::Queue;

let job = SendWelcomeEmail { user_id: 1 };
Queue::dispatch(&state.config.queue, job).unwrap();
```

## Running the Queue Worker

If you are using the `redis` driver, you need to run a worker process to consume jobs:

```bash
cargo run -- rune queue:work
```

You can specify a specific queue name:

```bash
cargo run -- rune queue:work --queue emails
```

## Production Deployment

Use **Supervisor** to keep your queue workers running:

```ini
[program:webrust-worker]
process_name=%(program_name)s_%(process_num)02d
command=/path/to/webrust rune queue:work
autostart=true
autorestart=true
user=www-data
numprocs=2
redirect_stderr=true
stdout_logfile=/path/to/webrust/storage/logs/worker.log
```

## Failed Jobs

WebRust includes built-in support for handling failed jobs. If a job throws an error during execution, it will be logged to the `failed_jobs` database table.

### Database Migration

To use this feature, you must run the migration to create the `failed_jobs` table:

```bash
cargo run -- rune migrate
```

The table structure includes:
- `connection`: The queue connection (e.g., redis)
- `queue`: The queue name
- `payload`: The JSON payload of the job
- `exception`: The error message returned by the job
- `failed_at`: Timestamp of failure

### Retrying Failed Jobs

(Coming Soon) Future versions will include a command to retry failed jobs from the database.
