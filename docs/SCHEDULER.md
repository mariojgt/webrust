# Task Scheduling

WebRust provides a robust task scheduler powered by `tokio-cron-scheduler`, with advanced features like preventing overlaps and distributed locking.

## Defining Schedules

You can define your scheduled tasks in `src/console/kernel.rs`. This file serves as the central location for all your scheduled commands, similar to Laravel's `app/Console/Kernel.php`.

```rust
use crate::services::scheduler::Scheduler;

pub async fn schedule(scheduler: &Scheduler) {
    // Basic schedule
    scheduler.job("0 * * * * *", || async {
        println!("Running every minute!");
    }).register().await.expect("Failed to register job");

    // Prevent Overlapping
    // If the job takes longer than 1 minute, the next run will be skipped.
    scheduler.job("0 * * * * *", || async {
        // Long running task
        tokio::time::sleep(std::time::Duration::from_secs(70)).await;
    })
    .name("long-task")
    .without_overlapping()
    .register().await.expect("Failed to register job");

    // On One Server
    // In a distributed deployment, ensure this only runs on one node.
    scheduler.job("0 0 * * * *", || async {
        println!("Running hourly on one server only.");
    })
    .name("hourly-report")
    .on_one_server()
    .register().await.expect("Failed to register job");
}
```

## Cron Syntax

WebRust uses standard cron syntax:

```
sec   min   hour   day of month   month   day of week   year
*     *     *      *              *       *             *
```

## Advanced Options

### `without_overlapping()`

By default, scheduled tasks will run even if the previous instance of the task is still running. To prevent this, you may use the `without_overlapping` method.

This requires a cache driver that supports atomic locks (Redis is recommended).

### `on_one_server()`

If your application is running on multiple servers, you may limit a scheduled job to only execute on a single server.

This requires a centralized cache driver (like Redis) to coordinate the lock.

### `name(string)`

When using `without_overlapping` or `on_one_server`, you **must** provide a unique name for the job so the lock key can be generated correctly.

## Running the Scheduler

To start the scheduler, run the following command:

```bash
cargo run -- rune schedule:run
```

This process should run in the background (e.g., using Supervisor or Docker).
