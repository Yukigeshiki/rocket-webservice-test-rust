use rocket::http::{ContentType, Status};
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{response, Request, Response};

/// Return when request is successful.
#[derive(Debug)]
pub struct SuccessResponse<T>
where
    T: Serialize,
{
    pub payload: Option<Json<T>>,
    pub status: Status,
}

impl<'r, T: Serialize> response::Responder<'r, 'r> for SuccessResponse<T> {
    fn respond_to(self, req: &'r Request) -> response::Result<'r> {
        Response::build_from(self.payload.respond_to(req)?)
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
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

impl<'r, T: Serialize> response::Responder<'r, 'r> for FailResponse<T> {
    fn respond_to(self, req: &'r Request) -> response::Result<'r> {
        Response::build_from(self.error.respond_to(req)?)
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}
