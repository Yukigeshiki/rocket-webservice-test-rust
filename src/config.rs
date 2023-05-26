use std::env;
use std::sync::OnceLock;

use dotenv::dotenv;

pub static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();

#[allow(dead_code)]
#[derive(Debug)]
pub struct AppConfig {
    log_level: String,
    env: String,
}

impl AppConfig {
    pub fn load() -> Self {
        if env::var("ENV").is_err() {
            dotenv().ok();
        }
        Self {
            log_level: env::var("LOG_LEVEL").unwrap_or("".to_string()),
            env: env::var("ENV").unwrap_or("dev".to_string()),
        }
    }

    pub fn get_log_level(&self) -> String {
        self.log_level.clone()
    }
}
