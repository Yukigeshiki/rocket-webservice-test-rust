use crate::common::{
    responses::{RouteResponse, Success},
    Logger,
};
use crate::models::{Id, IdPayload};

#[get("/<id>", format = "json")]
pub async fn get(mut logger: Logger, id: Id) -> RouteResponse<IdPayload> {
    logger.set_from("route::get");

    // ** do some cool stuff here (business logic, db fetch, etc.) ** //

    let payload = IdPayload { id };

    Ok(Success {
        req_id: logger.get_req_id(),
        payload,
    }
    .log_and_res(&logger))
}
