use rocket::{outcome, request, Request};
use std::fmt::{Debug, Formatter};
use uuid::Uuid;

#[derive(Debug)]
pub enum Error {}

#[allow(dead_code)]
struct Incoming<'r> {
    method: String,
    route: String,
    format: String,
    client_ip: String,
    req_id: &'r str,
}

impl Debug for Incoming<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} to {} format {} from {} request ID {}",
            self.method, self.route, self.format, self.client_ip, self.req_id
        )
    }
}

#[derive(Debug)]
pub struct ReqId(pub String);

#[rocket::async_trait]
impl<'r> request::FromRequest<'r> for ReqId {
    type Error = Error;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let req_id = Uuid::new_v4().to_string();
        println!(
            "{:?}",
            Incoming {
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
                req_id: &req_id
            }
        );
        outcome::Outcome::Success(ReqId(req_id))
    }
}
