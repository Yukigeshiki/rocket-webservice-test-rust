use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Route;

use crate::common::utils::Logger;
use crate::model::models::{Id, IdPayload, Message};
use crate::model::responses::{Fail, FailResponse, Success, SuccessResponse};

type F = FailResponse<Fail>;
type S<T> = SuccessResponse<Success<T>>;

pub fn routes() -> Vec<Route> {
    routes![get, post]
}

#[get("/<id>", format = "json")]
async fn get(logger: Logger, id: Id) -> Result<S<IdPayload>, F> {
    // ** do some cool stuff here (business logic, db fetch, etc.) ** //

    let payload = IdPayload { id };
    let res = Success {
        req_id: logger.0.clone(),
        payload,
    };
    logger.info(&res);

    Ok(SuccessResponse {
        payload: Some(Json(res)),
        status: Status::Ok,
    })
}

#[post("/hello", format = "json", data = "<msg>")]
async fn post(logger: Logger, msg: Json<Message<'_>>) -> Result<S<Message<'_>>, F> {
    let msg = msg.0;
    logger.info(&msg);

    // ** do some cool stuff here (business logic, db write, etc.) ** //

    let res = Success {
        req_id: logger.0.clone(),
        payload: msg,
    };
    logger.info(&res);

    Ok(SuccessResponse {
        payload: Some(Json(res)),
        status: Status::Ok,
    })
}
