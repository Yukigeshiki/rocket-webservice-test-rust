use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Catcher;
use uuid::Uuid;

use crate::models::Fail;
use crate::response::FailResponse;
use crate::utils::log;

pub fn supply_catchers() -> Vec<Catcher> {
    catchers![bad_request, not_found, server_error]
}

#[catch(400)]
pub fn bad_request() -> FailResponse<Fail<'static>> {
    let req_id = Uuid::new_v4().to_string();
    let res = Fail {
        req_id,
        error: "Client has issued a malformed or illegal request.",
    };

    log::error(&res);

    FailResponse {
        error: Some(Json(res)),
        status: Status::InternalServerError,
    }
}

#[catch(404)]
pub fn not_found() -> FailResponse<Fail<'static>> {
    let req_id = Uuid::new_v4().to_string();
    let res = Fail {
        req_id,
        error: "Resource was not found.",
    };

    log::error(&res);

    FailResponse {
        error: Some(Json(res)),
        status: Status::NotFound,
    }
}

#[catch(500)]
pub fn server_error() -> FailResponse<Fail<'static>> {
    let req_id = Uuid::new_v4().to_string();
    let res = Fail {
        req_id,
        error: "Internal server error has occurred.",
    };

    log::error(&res);

    FailResponse {
        error: Some(Json(res)),
        status: Status::InternalServerError,
    }
}
