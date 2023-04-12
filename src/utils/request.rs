use rocket::{outcome, request, Request};
use uuid::Uuid;

#[derive(Debug)]
pub struct RequestId(pub String);

#[derive(Debug)]
pub enum Error {}

#[rocket::async_trait]
impl<'r> request::FromRequest<'r> for RequestId {
    type Error = Error;

    async fn from_request(_request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        outcome::Outcome::Success(RequestId(Uuid::new_v4().to_string()))
    }
}
