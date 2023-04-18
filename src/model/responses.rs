//! These are the default response models. Leave them as is, or update them to fit your needs.

use std::fmt::{Debug, Formatter};

use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Result};
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{Request, Response};

/// Return when request is successful.
#[derive(Debug)]
pub struct SuccessResponse<T>
where
    T: Serialize,
{
    pub payload: Option<Json<T>>,
    pub status: Status,
}

impl<'r, T: Serialize> Responder<'r, 'r> for SuccessResponse<T> {
    fn respond_to(self, req: &'r Request) -> Result<'r> {
        Response::build_from(self.payload.respond_to(req)?)
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

/// Contains a successful requests payload and request ID.
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Success<T>
where
    T: Serialize,
{
    #[serde(rename = "reqId")]
    pub req_id: String,
    pub payload: T,
}

impl<T> Debug for Success<T>
where
    T: Debug + Serialize,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ \"req_id\": \"{}\", \"payload\": {:?} }}",
            self.req_id, self.payload
        )
    }
}

/// Return when request has failed.
#[derive(Debug)]
pub struct FailResponse<T>
where
    T: Serialize,
{
    pub error: Option<Json<T>>,
    pub status: Status,
}

impl<'r, T: Serialize> Responder<'r, 'r> for FailResponse<T> {
    fn respond_to(self, req: &'r Request) -> Result<'r> {
        Response::build_from(self.error.respond_to(req)?)
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

/// Contains a failed requests error message and request ID.
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Fail {
    #[serde(rename = "reqId")]
    pub req_id: String,
    pub error: &'static str,
    pub code: u32,
}

impl Debug for Fail {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ \"req_id\": \"{}\", \"error\": \"{}\", \"code\": \"{}\" }}",
            self.req_id, self.error, self.code
        )
    }
}
