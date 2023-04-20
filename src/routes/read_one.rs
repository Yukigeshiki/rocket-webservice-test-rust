use mongodb::bson::{doc, oid::ObjectId};
use rocket::State;

use crate::common::{
    responses::{Fail, RouteResponse, Success},
    Logger,
};
use crate::models::{Id, Message};
use crate::repository::db_services::DatabaseService;
use crate::routes::Error;
use crate::AppState;

#[get("/<id>", format = "json")]
pub async fn read_one(
    mut logger: Logger,
    state: &State<AppState>,
    id: Id,
) -> RouteResponse<Message> {
    logger.set_from("route::read_one");
    let _id = ObjectId::parse_str(&id).map_err(|err| {
        let error = Error::IncorrectId(err);
        let status = error.get_status();
        Fail {
            req_id: logger.get_req_id(),
            error: error.get_message(),
            code: status.code,
        }
        .log_and_res(&logger, status)
    })?;

    let result = state
        .mongo
        .read_one(doc! {"_id": _id})
        .await
        .map_err(|err| {
            let error = Error::QueryFailed(err);
            let status = error.get_status();
            Fail {
                req_id: logger.get_req_id(),
                error: error.get_message(),
                code: status.code,
            }
            .log_and_res(&logger, status)
        })?;

    match result {
        Some(msg) => Ok(Success {
            req_id: logger.get_req_id(),
            payload: msg,
        }
        .log_and_res(&logger)),
        None => {
            let error = Error::DocumentNotFound(id);
            let status = error.get_status();
            Err(Fail {
                req_id: logger.get_req_id(),
                error: error.get_message(),
                code: status.code,
            }
            .log_and_res(&logger, status))
        }
    }
}
