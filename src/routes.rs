use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Route;

use crate::models::{Fail, Id, IdPayload, Message, Success};
use crate::utils::request::RequestId;
use crate::utils::response::{FailResponse, SuccessResponse};

pub fn supply_routes() -> Vec<Route> {
    routes![get, post]
}

#[get("/<id>", format = "json")]
pub async fn get(
    req_id: RequestId,
    id: Id,
) -> Result<SuccessResponse<Success<IdPayload>>, FailResponse<Fail<'static>>> {
    let req_id = req_id.0;
    let payload = IdPayload { id }; // fetch something from db

    Ok(SuccessResponse {
        payload: Some(Json(Success { req_id, payload })),
        status: Status::Ok,
    })
}

#[post("/hello", format = "json", data = "<msg>")]
pub async fn post(
    req_id: RequestId,
    msg: Json<Message<'_>>,
) -> Result<SuccessResponse<Success<Message<'_>>>, FailResponse<Fail<'static>>> {
    let req_id = req_id.0;
    let payload = msg.0; // add something to db

    Ok(SuccessResponse {
        payload: Some(Json(Success { req_id, payload })),
        status: Status::Ok,
    })
}
