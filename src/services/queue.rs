use serde::{Serialize, Deserialize};
use crate::config::queue::QueueConfig;
use redis::Commands;
use async_trait::async_trait;

#[async_trait]
pub trait Job: Serialize + for<'de> Deserialize<'de> + Send + Sync {
    async fn handle(&self) -> Result<(), String>;
    fn name(&self) -> String;
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

    pub async fn work(config: &QueueConfig) -> Result<(), String> {
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
                            println!("📥 Processing job: {}", json_str);
                            // In a real implementation, we would deserialize and execute the job here.
                            // For now, we just acknowledge receipt.
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
}
