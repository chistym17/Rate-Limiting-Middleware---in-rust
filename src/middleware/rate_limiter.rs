use redis::AsyncCommands;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct RateLimiter {
    redis_client: Arc<Mutex<redis::aio::Connection>>,  
    max_requests: usize,
    time_window: usize,
}

impl RateLimiter {
    pub async fn new(redis_url: &str, max_requests: usize, time_window: usize) -> Self {

        let client = redis::Client::open(redis_url).expect("Invalid Redis URL");
        let connection = client.get_tokio_connection().await.expect("Failed to connect to Redis");

        Self {
            redis_client: Arc::new(Mutex::new(connection)),
            max_requests,
            time_window,
        }
    }

    pub async fn check_rate_limit(&self, ip: &str) -> Result<bool, redis::RedisError> {
        let mut conn = self.redis_client.lock().await;  
        
        let key = format!("rate_limit:{}", ip);
        let request_count: usize = conn.incr(&key, 1).await?;
        if request_count == 1 {
            conn.expire(&key, self.time_window).await?;
        }

        Ok(request_count > self.max_requests)
    }
}
