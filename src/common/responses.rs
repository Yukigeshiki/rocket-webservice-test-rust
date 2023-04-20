//! These are the default response models. Leave them as is, or modify them to fit your needs. Debug
//! implementations are JSON formatted for easy to analyse log outputs.

use std::fmt::{Debug, Formatter};

use rocket::{
    http::{ContentType, Status},
    response::{Responder, Result},
    serde::{
        json::{serde_json, Json},
        Serialize,
    },
    Request,
};

use crate::common::logger::Logger;

/// Generates a Rocket response type by implementing the Responder trait.
macro_rules! generate_response_type {
    ($res_type:ident, $field:ident) => {
        #[derive(Debug)]
        pub struct $res_type<T>
        where
            T: Debug + Serialize,
        {
            pub $field: Option<Json<T>>,
            pub status: Status,
        }

        impl<'r, T: Debug + Serialize> Responder<'r, 'r> for $res_type<T> {
            fn respond_to(self, req: &'r Request) -> Result<'r> {
                rocket::Response::build_from(self.$field.respond_to(req)?)
                    .status(self.status)
                    .header(ContentType::JSON)
                    .ok()
            }
        }
    };
}

/// Implements the Debug trait for a response.
macro_rules! implement_response_debug {
    ($res:ident) => {
        impl<T: Debug + Serialize> Debug for $res<T> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(self).unwrap_or("Not available".into())
                )
            }
        }
    };
}

pub type RouteResponse<T> =
    std::result::Result<SuccessResponse<Success<T>>, FailResponse<Fail<String>>>;
pub type CatcherResponse = FailResponse<Fail<&'static str>>;

// Return when request is successful.
generate_response_type!(SuccessResponse, payload);

/// Contains a successful requests payload and request ID.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct Success<T>
where
    T: Debug + Serialize,
{
    pub req_id: String,
    pub payload: T,
}

implement_response_debug!(Success);

impl<T: Debug + Serialize> Success<T> {
    /// Log and return successful response.
    pub fn log_and_res(self, logger: &Logger) -> SuccessResponse<Success<T>> {
        logger.info(&self);
        SuccessResponse {
            payload: Some(Json(self)),
            status: Status::Ok,
        }
    }
}

// Return when request has failed.
generate_response_type!(FailResponse, error);

/// Contains a failed requests error message, code, and request ID.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct Fail<T>
where
    T: Serialize + Debug,
{
    pub req_id: String,
    pub error: T,
    pub code: u16,
}

implement_response_debug!(Fail);

impl<T: Serialize + Debug> Fail<T> {
    /// Log and return failed response.
    pub fn log_and_res(self, logger: &Logger, status: Status) -> FailResponse<Fail<T>> {
        logger.error(&self);
        FailResponse {
            error: Some(Json(self)),
            status,
        }
    }
}
