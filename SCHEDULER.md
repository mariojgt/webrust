# ⏰ Task Scheduling

WebRust includes a robust task scheduler powered by `tokio-cron-scheduler`, allowing you to schedule recurring tasks using a fluent syntax or standard cron expressions.

## Defining Schedules

Schedules are defined in `src/main.rs` (or you can move them to a dedicated `src/console/kernel.rs` if you prefer).

Look for the `RuneCommand::ScheduleRun` match arm:

```rust
RuneCommand::ScheduleRun => {
    println!("⏰ Starting Scheduler...");
    let scheduler = crate::services::scheduler::Scheduler::new().await.expect("Failed to create scheduler");

    // Run a closure every 10 seconds
    scheduler.add_async("1/10 * * * * *", || async {
        println!("Tick! (every 10s)");
        // You can dispatch jobs here too!
        // Queue::dispatch(&config, MyJob { ... });
    }).await.expect("Failed to add task");

    scheduler.start().await.expect("Failed to start scheduler");

    // Keep alive
    tokio::signal::ctrl_c().await?;
    println!("🛑 Scheduler stopped");
}
```

## Cron Expressions

The scheduler uses standard cron syntax with an optional seconds field at the beginning:

| Expression | Meaning |
| :--- | :--- |
| `1/10 * * * * *` | Every 10 seconds |
| `0 * * * * *` | Every minute |
| `0 0 * * * *` | Every hour |
| `0 0 0 * * *` | Every day at midnight |
| `0 0 0 * * 0` | Every Sunday at midnight |

## Running the Scheduler

To start the scheduler, run the following command:

```bash
cargo run -- rune schedule:run
```

This process should run in the background (e.g., using Supervisor or Docker).

## Production Setup

In production, you should use a process monitor like **Supervisor** to keep the scheduler running.

```ini
[program:webrust-scheduler]
process_name=%(program_name)s
command=/path/to/webrust rune schedule:run
autostart=true
autorestart=true
user=www-data
redirect_stderr=true
stdout_logfile=/path/to/webrust/storage/logs/scheduler.log
```
