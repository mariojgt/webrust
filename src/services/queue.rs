use serde::{Serialize, Deserialize};
use crate::config::queue::QueueConfig;
use crate::database::DatabaseManager;
use redis::Commands;
use async_trait::async_trait;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use chrono::Local;

#[async_trait]
pub trait Job: Serialize + for<'de> Deserialize<'de> + Send + Sync {
    async fn handle(&self) -> Result<(), String>;
    fn name(&self) -> String;
}

// Type alias for the handler function
type JobHandler = Box<dyn Fn(String) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>> + Send + Sync>;

pub struct JobRegistry {
    handlers: HashMap<String, JobHandler>,
}

impl JobRegistry {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register<J: Job + 'static + Clone>(&mut self, name: &str) {
        self.handlers.insert(name.to_string(), Box::new(|payload: String| {
            Box::pin(async move {
                let job: J = serde_json::from_str(&payload).map_err(|e| format!("Deserialization error: {}", e))?;
                job.handle().await
            })
        }));
    }

    pub async fn execute(&self, name: &str, payload: String) -> Result<(), String> {
        if let Some(handler) = self.handlers.get(name) {
            handler(payload).await
        } else {
            Err(format!("No handler registered for job: {}", name))
        }
    }
}

pub struct Queue;

impl Queue {
    pub fn dispatch<J: Job + 'static>(config: &QueueConfig, job: J) -> Result<(), String> {
        match config.driver.as_str() {
            "sync" => {
                tokio::spawn(async move {
                    if let Err(e) = job.handle().await {
                        eprintln!("Job failed: {}", e);
                    }
                });
                Ok(())
            },
            "redis" => {
                let client = redis::Client::open(config.redis_url.as_str()).map_err(|e| e.to_string())?;
                let mut con = client.get_connection().map_err(|e| e.to_string())?;

                // We wrap the job in a structure that identifies it
                let job_name = job.name();
                let payload = serde_json::to_string(&job).map_err(|e| e.to_string())?;

                let json = serde_json::json!({
                    "job": job_name,
                    "payload": payload
                });

                let _: () = con.rpush(&config.queue_name, json.to_string()).map_err(|e| e.to_string())?;
                Ok(())
            },
            _ => Err("Unknown queue driver".to_string()),
        }
    }

    pub async fn work(config: &QueueConfig, registry: Arc<JobRegistry>, db_manager: Option<DatabaseManager>) -> Result<(), String> {
        println!("👷 Starting queue worker for queue: {}", config.queue_name);

        match config.driver.as_str() {
            "redis" => {
                let client = redis::Client::open(config.redis_url.as_str()).map_err(|e| e.to_string())?;
                let mut con = client.get_connection().map_err(|e| e.to_string())?;

                loop {
                    // BLPOP blocks until a job is available
                    // redis crate blpop returns a tuple (key, value)
                    let result: redis::RedisResult<(String, String)> = con.blpop(&config.queue_name, 0.0);

                    match result {
                        Ok((_list, json_str)) => {
                            // Parse the JSON wrapper
                            let wrapper: serde_json::Value = match serde_json::from_str(&json_str) {
                                Ok(v) => v,
                                Err(e) => {
                                    eprintln!("❌ Invalid JSON in queue: {}", e);
                                    continue;
                                }
                            };

                            let job_name = match wrapper.get("job").and_then(|v| v.as_str()) {
                                Some(n) => n,
                                None => {
                                    eprintln!("❌ Job name missing in queue item");
                                    continue;
                                }
                            };

                            let payload = match wrapper.get("payload").and_then(|v| v.as_str()) {
                                Some(p) => p.to_string(),
                                None => {
                                    // Fallback: maybe the payload is the object itself if not stringified?
                                    // For now assume stringified as per dispatch
                                    eprintln!("❌ Payload missing in queue item");
                                    continue;
                                }
                            };

                            println!("📥 Processing job: {}", job_name);

                            match registry.execute(job_name, payload.clone()).await {
                                Ok(_) => println!("✅ Job {} completed", job_name),
                                Err(e) => {
                                    eprintln!("❌ Job {} failed: {}", job_name, e);
                                    if let Some(db) = &db_manager {
                                        Self::log_failed_job(db, config, job_name, &payload, &e).await;
                                    }
                                },
                            }
                        }
                        Err(e) => {
                            eprintln!("Redis error: {}", e);
                            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                        }
                    }
                }
            },
            _ => {
                println!("Queue driver '{}' does not support worker process (sync runs immediately).", config.driver);
                Ok(())
            }
        }
    }

    async fn log_failed_job(db_manager: &DatabaseManager, config: &QueueConfig, job_name: &str, payload: &str, exception: &str) {
        if let Some(pool) = db_manager.default_connection() {
            let sql = "INSERT INTO failed_jobs (connection, queue, payload, exception, failed_at) VALUES (?, ?, ?, ?, ?)";
            let now = Local::now().naive_local();

            let res = sqlx::query(sql)
                .bind(&config.driver)
                .bind(&config.queue_name)
                .bind(payload)
                .bind(exception)
                .bind(now)
                .execute(pool)
                .await;

            if let Err(e) = res {
                eprintln!("❌ Failed to log failed job to database: {}", e);
            } else {
                println!("📝 Failed job logged to database");
            }
        }
    }
}
