//! Add your utilities here, e.g. logger, helper functions, etc.

use std::fmt::{Debug, Formatter};

use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use uuid::Uuid;

pub struct Logger(pub String);

impl Logger {
    pub fn log<T>(&self, output: &T)
    where
        T: Debug,
    {
        println!("{:?} request ID {}", output, self.0)
    }

    pub fn info<T>(&self, output: &T)
    where
        T: Debug,
    {
        println!("INFO {:?} request ID {}", output, self.0)
    }

    pub fn error<T>(&self, output: &T)
    where
        T: Debug,
    {
        eprintln!("ERROR {:?} request ID {}", output, self.0)
    }
}

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
        let req_id = Uuid::new_v4().to_string();
        let logger = Self(req_id);

        logger.log(&Incoming {
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

        Outcome::Success(logger)
    }
}
