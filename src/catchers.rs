use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Catcher;
use uuid::Uuid;

use crate::models::Fail;
use crate::utils::response::FailResponse;

pub fn supply_catchers() -> Vec<Catcher> {
    catchers![not_found, server_error]
}

#[catch(404)]
pub fn not_found() -> FailResponse<Fail<'static>> {
    let req_id = Uuid::new_v4().to_string();

    FailResponse {
        error: Some(Json(Fail {
            req_id,
            error: "Resource was not found.",
        })),
        status: Status::NotFound,
    }
}

#[catch(500)]
pub fn server_error() -> FailResponse<Fail<'static>> {
    let req_id = Uuid::new_v4().to_string();

    FailResponse {
        error: Some(Json(Fail {
            req_id,
            error: "An internal server error has occurred.",
        })),
        status: Status::InternalServerError,
    }
}
