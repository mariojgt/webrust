use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use crate::cache::Cache;
use std::sync::Arc;

pub struct Scheduler {
    sched: JobScheduler,
    cache: Cache,
}

impl Scheduler {
    pub async fn new(cache: Cache) -> Result<Self, JobSchedulerError> {
        let sched = JobScheduler::new().await?;
        Ok(Self { sched, cache })
    }

    pub fn job<F, Fut>(&self, cron: &str, job: F) -> JobBuilder
    where
        F: Fn() -> Fut + Send + Sync + 'static + Clone,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        JobBuilder {
            scheduler: self,
            cron: cron.to_string(),
            job: Box::new(move || {
                let job = job.clone();
                Box::pin(async move {
                    job().await;
                })
            }),
            without_overlapping: false,
            on_one_server: false,
            name: None,
        }
    }

    pub async fn start(&self) -> Result<(), JobSchedulerError> {
        self.sched.start().await
    }
}

pub struct JobBuilder<'a> {
    scheduler: &'a Scheduler,
    cron: String,
    job: Box<dyn Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> + Send + Sync>,
    without_overlapping: bool,
    on_one_server: bool,
    name: Option<String>,
}

impl<'a> JobBuilder<'a> {
    pub fn without_overlapping(mut self) -> Self {
        self.without_overlapping = true;
        self
    }

    pub fn on_one_server(mut self) -> Self {
        self.on_one_server = true;
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub async fn register(self) -> Result<(), JobSchedulerError> {
        let cache = self.scheduler.cache.clone();
        let job_fn = self.job;
        let without_overlapping = self.without_overlapping;
        let on_one_server = self.on_one_server;
        let name = self.name.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let cron = self.cron.clone();

        self.scheduler.sched.add(Job::new_async(cron.as_str(), move |_uuid, _l| {
            let cache = cache.clone();
            let job_fn = job_fn(); // Get the future
            let name = name.clone();

            Box::pin(async move {
                let lock_key = format!("scheduler:lock:{}", name);

                if on_one_server {
                    // Try to acquire lock for the minute (assuming cron runs minutely at most)
                    // Or just a short duration to prevent other servers from picking it up
                    if let Ok(true) = cache.add(&lock_key, "locked", 60).await {
                        // Acquired lock, run job
                        job_fn.await;
                    }
                } else if without_overlapping {
                    // Try to acquire lock for a long time (expires when job done)
                    // We set a long expiry (e.g. 1 hour) just in case of crash
                    if let Ok(true) = cache.add(&lock_key, "processing", 3600).await {
                        job_fn.await;
                        // Release lock
                        let _ = cache.forget(&lock_key).await;
                    }
                } else {
                    // Normal execution
                    job_fn.await;
                }
            })
        })?).await?;

        Ok(())
    }
}
