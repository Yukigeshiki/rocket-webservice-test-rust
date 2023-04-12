use rocket::http::{ContentType, Status};
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{response, Request, Response};

/// The struct we return for success responses (200s)
#[derive(Debug)]
pub struct SuccessResponse<T>
where
    T: Serialize,
{
    pub payload: Option<Json<T>>,
    pub status: Status,
}

/// Implements the `Responder` trait for Rocket, so we can simply return for
/// endpoint functions, result and Rocket takes care of the rest.
impl<'r, T: Serialize> response::Responder<'r, 'r> for SuccessResponse<T> {
    fn respond_to(self, req: &'r Request) -> response::Result<'r> {
        Response::build_from(self.payload.respond_to(req)?)
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

/// The struct we return for error responses (400s, 500s)
#[derive(Debug)]
pub struct FailResponse<T>
where
    T: Serialize,
{
    pub error: Option<Json<T>>,
    pub status: Status,
}

/// Implements the `Responder` trait, much like for `ApiResponse`, but for `ApiError`
impl<'r, T: Serialize> response::Responder<'r, 'r> for FailResponse<T> {
    fn respond_to(self, req: &'r Request) -> response::Result<'r> {
        Response::build_from(self.error.respond_to(req)?)
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}
