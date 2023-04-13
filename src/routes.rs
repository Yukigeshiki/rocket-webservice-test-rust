use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Route;

use crate::models::{Fail, Id, IdPayload, Message, Success};
use crate::request::ReqId;
use crate::response::{FailResponse, SuccessResponse};
use crate::utils::log;

type F = FailResponse<Fail<'static>>;
type S<T> = SuccessResponse<Success<T>>;

pub fn supply_routes() -> Vec<Route> {
    routes![get, post]
}

#[get("/<id>", format = "json")]
pub async fn get(req_id: ReqId, id: Id) -> Result<S<IdPayload>, F> {
    let req_id = req_id.0;
    let payload = IdPayload { id }; // fetch something from the db
    let res = Success { req_id, payload };

    log::info(&res);

    Ok(SuccessResponse {
        payload: Some(Json(res)),
        status: Status::Ok,
    })
}

#[post("/hello", format = "json", data = "<msg>")]
pub async fn post(req_id: ReqId, msg: Json<Message<'_>>) -> Result<S<Message<'_>>, F> {
    let req_id = req_id.0;
    let payload = msg.0; // add something to the db
    let res = Success { req_id, payload };

    log::info(&res);

    Ok(SuccessResponse {
        payload: Some(Json(res)),
        status: Status::Ok,
    })
}
