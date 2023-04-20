//! Add your app config here.

use std::env;
use std::sync::OnceLock;

use dotenv::dotenv;

pub static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();

#[allow(dead_code)]
#[derive(Debug)]
pub struct AppConfig {
    mongo_uri: String,
    log_level: String,
    env: String,
}

impl AppConfig {
    pub fn load() -> Self {
        if env::var("ENV").is_err() {
            dotenv().ok();
        }
        Self {
            mongo_uri: env::var("MONGO_URI").unwrap_or("mongodb://127.0.0.1:27017".to_string()),
            log_level: env::var("LOG_LEVEL").unwrap_or("".to_string()),
            env: env::var("ENV").unwrap_or("dev".to_string()),
        }
    }

    pub fn get_mongo_uri(&self) -> String {
        self.mongo_uri.clone()
    }

    pub fn get_log_level(&self) -> String {
        self.log_level.clone()
    }
}
