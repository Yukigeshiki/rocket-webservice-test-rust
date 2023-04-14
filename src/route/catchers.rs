use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Catcher;
use uuid::Uuid;

use crate::model::responses::{Fail, FailResponse};
use crate::util::logger::Logger;

pub fn catchers() -> Vec<Catcher> {
    catchers![bad_request, not_found, server_error]
}

#[catch(500)]
fn server_error() -> FailResponse<Fail> {
    let req_id = Uuid::new_v4().to_string();
    let res = Fail {
        req_id: req_id.clone(),
        error: "Internal server error has occurred.",
        code: 500,
    };

    Logger(req_id).error(&res);

    FailResponse {
        error: Some(Json(res)),
        status: Status::InternalServerError,
    }
}

#[catch(404)]
fn not_found() -> FailResponse<Fail> {
    let req_id = Uuid::new_v4().to_string();
    let res = Fail {
        req_id: req_id.clone(),
        error: "Resource was not found.",
        code: 404,
    };

    Logger(req_id).error(&res);

    FailResponse {
        error: Some(Json(res)),
        status: Status::NotFound,
    }
}

#[catch(400)]
fn bad_request() -> FailResponse<Fail> {
    let req_id = Uuid::new_v4().to_string();
    let res = Fail {
        req_id: req_id.clone(),
        error: "Client has issued a malformed or illegal request.",
        code: 400,
    };

    Logger(req_id).error(&res);

    FailResponse {
        error: Some(Json(res)),
        status: Status::InternalServerError,
    }
}
