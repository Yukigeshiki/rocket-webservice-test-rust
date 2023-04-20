//! Add your logger implementation here (or just use the default logger).

use std::fmt::{Debug, Formatter};

use rocket::{
    request::{FromRequest, Outcome},
    Request,
};
use uuid::Uuid;

use crate::common::DateFormatter;
use crate::config::APP_CONFIG;

/// A Logger instance holds a request ID, the log level and the name of the function or module the
/// logger is logging from. The levels are "debug", "info" and "error". If the log level is an empty
/// string the default is "debug" - any other value other than those listed will be considered
/// "off". The `log` call will always log output regardless of the log level.
pub struct Logger {
    req_id: String,
    log_level: String,
    from: &'static str,
}

// Update timezone in logger string.
#[allow(dead_code)]
impl Logger {
    pub fn new(req_id: String, log_level: String, from: &'static str) -> Self {
        Self {
            req_id,
            log_level,
            from,
        }
    }

    pub fn get_req_id(&self) -> String {
        self.req_id.clone()
    }

    pub fn set_from(&mut self, from: &'static str) {
        self.from = from;
    }

    pub fn log<T>(&self, output: &T)
    where
        T: Debug + ?Sized,
    {
        println!("{:?} at {} SAST", output, Self::now_and_format())
    }

    pub fn debug<T>(&self, output: &T)
    where
        T: Debug + ?Sized,
    {
        let level = self.log_level.to_lowercase();
        if level == "debug" || level.is_empty() {
            println!(
                "DEBUG: {:?} - logged from {} with request ID {} at {} SAST",
                output,
                self.from,
                self.req_id,
                Self::now_and_format()
            )
        }
    }

    pub fn info<T>(&self, output: &T)
    where
        T: Debug + ?Sized,
    {
        let level = self.log_level.to_lowercase();
        if level == "debug" || level == "info" || level.is_empty() {
            println!(
                "INFO: {:?} - logged from {} with request ID {} at {} SAST",
                output,
                self.from,
                self.req_id,
                Self::now_and_format()
            )
        }
    }

    pub fn error<T>(&self, output: &T)
    where
        T: Debug + ?Sized,
    {
        let level = self.log_level.to_lowercase();
        if level == "debug" || level == "info" || level == "error" || level.is_empty() {
            eprintln!(
                "ERROR: {:?} - logged from {} with request ID {} at {} SAST",
                output,
                self.from,
                self.req_id,
                Self::now_and_format()
            )
        }
    }
}

impl DateFormatter for Logger {}

#[derive(Debug)]
pub enum Error {}

#[allow(dead_code)]
struct Incoming {
    method: String,
    route: String,
    format: String,
    client_ip: String,
}

impl Debug for Incoming {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} to {} format {} from {}",
            self.method, self.route, self.format, self.client_ip
        )
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Logger {
    type Error = Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let mut logger = Self::new(
            Uuid::new_v4().to_string(),
            APP_CONFIG.get().unwrap().get_log_level(),
            "logger::from_request",
        );
        logger.info(&Incoming {
            method: req.method().to_string(),
            route: req.uri().to_string(),
            format: match req.format() {
                Some(f) => f.to_string(),
                None => "\"Format Not Available\"".to_string(),
            },
            client_ip: match req.client_ip() {
                Some(ip) => ip.to_string(),
                None => "\"IP Address Not Available\"".to_string(),
            },
        });
        logger.set_from("\"Not Available\"");
        Outcome::Success(logger)
    }
}
