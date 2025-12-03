use crate::services::scheduler::Scheduler;

pub async fn schedule(scheduler: &Scheduler) {
    // Example task
    scheduler.job("1/10 * * * * *", || async {
        println!("Tick! (every 10s)");
    })
    .name("tick_task")
    .without_overlapping()
    .register().await.expect("Failed to add task");
}
