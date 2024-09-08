use std::env;

pub fn redis_url() -> String {
    env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string())
}

pub const RATE_LIMIT: u32 = 10; 
