use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

pub struct Scheduler {
    sched: JobScheduler,
}

impl Scheduler {
    pub async fn new() -> Result<Self, JobSchedulerError> {
        let sched = JobScheduler::new().await?;
        Ok(Self { sched })
    }

    pub async fn add_async<F, Fut>(&self, cron: &str, job: F) -> Result<(), JobSchedulerError>
    where
        F: Fn() -> Fut + Send + Sync + 'static + Clone,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        self.sched.add(Job::new_async(cron, move |_uuid, _l| {
            let job = job.clone();
            Box::pin(async move {
                job().await;
            })
        })?).await?;
        Ok(())
    }

    pub async fn start(&self) -> Result<(), JobSchedulerError> {
        self.sched.start().await
    }
}
