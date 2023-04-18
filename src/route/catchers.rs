//! Add your error catchers here - as many as your project might need.

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Catcher;
use uuid::Uuid;

use crate::common::utils::Logger;
use crate::model::responses::{Fail, FailResponse};

pub fn catchers() -> Vec<Catcher> {
    catchers![
        internal_server_error,
        unprocessable_entity,
        not_found,
        bad_request
    ]
}

#[catch(500)]
fn internal_server_error() -> FailResponse<Fail> {
    let status = Status::InternalServerError;
    let req_id = Uuid::new_v4().to_string();
    let res = Fail {
        req_id: req_id.clone(),
        error: "An internal server error has occurred.",
        code: status.code,
    };

    Logger(req_id).error(&res);

    FailResponse {
        error: Some(Json(res)),
        status,
    }
}

#[catch(422)]
fn unprocessable_entity() -> FailResponse<Fail> {
    let status = Status::UnprocessableEntity;
    let req_id = Uuid::new_v4().to_string();
    let res = Fail {
        req_id: req_id.clone(),
        error: "The request was well-formed but was unable to be followed due to semantic errors.",
        code: status.code,
    };

    Logger(req_id).error(&res);

    FailResponse {
        error: Some(Json(res)),
        status,
    }
}

#[catch(404)]
fn not_found() -> FailResponse<Fail> {
    let status = Status::NotFound;
    let req_id = Uuid::new_v4().to_string();
    let res = Fail {
        req_id: req_id.clone(),
        error: "The resource was not found.",
        code: status.code,
    };

    Logger(req_id).error(&res);

    FailResponse {
        error: Some(Json(res)),
        status,
    }
}

#[catch(400)]
fn bad_request() -> FailResponse<Fail> {
    let status = Status::BadRequest;
    let req_id = Uuid::new_v4().to_string();
    let res = Fail {
        req_id: req_id.clone(),
        error: "The client has issued a malformed or illegal request.",
        code: status.code,
    };

    Logger(req_id).error(&res);

    FailResponse {
        error: Some(Json(res)),
        status,
    }
}
