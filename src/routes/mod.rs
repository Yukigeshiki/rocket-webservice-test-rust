//! Add your HTTP routes here.

use rocket::Route;

use crate::routes::read_one::read_one as read_one_route;
use crate::routes::write_one::write_one as write_one_route;
use mongodb::bson;
use rocket::http::Status;

use crate::models::Id;

mod read_one;
mod write_one;

pub enum Error {
    IncorrectId(bson::oid::Error),
    QueryFailed(mongodb::error::Error),
    DocumentNotFound(Id),
}

impl Error {
    pub fn get_message(&self) -> String {
        match self {
            Self::IncorrectId(err) => format!("Incorrect ID: {err}"),
            Self::QueryFailed(err) => format!("Mongo query failed with error: {err}"),
            Self::DocumentNotFound(id) => {
                format!("Document with _id: \"{id}\" could not be found.")
            }
        }
    }

    pub fn get_status(&self) -> Status {
        match self {
            Self::IncorrectId(_) => Status::BadRequest,
            Self::QueryFailed(_) => Status::InternalServerError,
            Self::DocumentNotFound(_) => Status::NotFound,
        }
    }
}

pub fn routes() -> Vec<Route> {
    routes![read_one_route, write_one_route]
}
