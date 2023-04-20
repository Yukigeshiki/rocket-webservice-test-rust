//! Add your error catchers here - as many as your project might need.

use rocket::http::Status;
use rocket::Catcher;
use uuid::Uuid;

use crate::common::{
    logger::Logger,
    responses::{CatcherResponse, Fail},
};
use crate::config::APP_CONFIG;

/// Errors used by catchers.
#[allow(clippy::enum_variant_names)]
pub enum Error {
    InternalServerError,
    UnprocessableEntity,
    NotFound,
    BadRequest,
}

impl Error {
    pub fn get_message(&self) -> &'static str {
        match self {
            Self::InternalServerError => "An internal server error has occurred.",
            Self::UnprocessableEntity => {
                "The request was well-formed but was unable to be followed due to semantic errors."
            }
            Self::NotFound => "The resource was not found.",
            Self::BadRequest => "The client has issued a malformed or illegal request.",
        }
    }

    pub fn get_status(&self) -> Status {
        match self {
            Self::InternalServerError => Status::InternalServerError,
            Self::UnprocessableEntity => Status::UnprocessableEntity,
            Self::NotFound => Status::NotFound,
            Self::BadRequest => Status::BadRequest,
        }
    }
}

pub fn catchers() -> Vec<Catcher> {
    catchers![
        internal_server_error,
        unprocessable_entity,
        not_found,
        bad_request
    ]
}

#[catch(500)]
pub fn internal_server_error() -> CatcherResponse {
    respond_with(Error::InternalServerError)
}

#[catch(422)]
pub fn unprocessable_entity() -> CatcherResponse {
    respond_with(Error::UnprocessableEntity)
}

#[catch(404)]
pub fn not_found() -> CatcherResponse {
    respond_with(Error::NotFound)
}

#[catch(400)]
pub fn bad_request() -> CatcherResponse {
    respond_with(Error::BadRequest)
}

fn respond_with(error: Error) -> CatcherResponse {
    let logger = Logger::new(
        Uuid::new_v4().to_string(),
        APP_CONFIG.get().unwrap().get_log_level(),
        "catcher",
    );
    let status = error.get_status();
    Fail {
        req_id: logger.get_req_id(),
        error: error.get_message(),
        code: status.code,
    }
    .log_and_res(&logger, status)
}
