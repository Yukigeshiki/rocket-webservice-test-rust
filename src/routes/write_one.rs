use mongodb::bson::doc;
use rocket::{serde::json::Json, State};

use crate::common::{
    responses::{Fail, RouteResponse, Success},
    Logger,
};
use crate::models::{message::Message, Model};
use crate::repository::db_services::DatabaseService;
use crate::routes::Error;
use crate::AppState;

#[post("/hello", format = "json", data = "<msg>")]
pub async fn write_one(
    mut logger: Logger,
    state: &State<AppState>,
    msg: Json<Message>,
) -> RouteResponse<Message> {
    let mut msg = msg.0;
    logger.set_from("route::write_one");
    logger.info(&msg);

    msg.set_created_at();
    msg.set_id();

    state.mongo.write_one(&msg).await.map_err(|err| {
        let error = Error::QueryFailed(err);
        let status = error.get_status();
        Fail {
            req_id: logger.get_req_id(),
            error: error.get_message(),
            code: status.code,
        }
        .log_and_res(&logger, status)
    })?;

    Ok(Success {
        req_id: logger.get_req_id(),
        payload: msg,
    }
    .log_and_res(&logger))
}
