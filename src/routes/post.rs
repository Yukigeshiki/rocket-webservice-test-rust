use rocket::serde::json::Json;

use crate::common::{
    responses::{RouteResponse, Success},
    Logger,
};
use crate::models::Message;

#[post("/hello", format = "json", data = "<msg>")]
pub async fn post(mut logger: Logger, msg: Json<Message<'_>>) -> RouteResponse<Message<'_>> {
    let msg = msg.0;
    logger.set_from("route::post");
    logger.info(&msg);

    // ** do some cool stuff here (business logic, db write, etc.) ** //

    Ok(Success {
        req_id: logger.get_req_id(),
        payload: msg,
    }
    .log_and_res(&logger))
}
