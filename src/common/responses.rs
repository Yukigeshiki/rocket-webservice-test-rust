//! These are the default response models. Leave them as is, or modify them to fit your needs. Debug
//! implementations are JSON formatted for easy to analyse log outputs.

use std::fmt::{Debug, Formatter};

use async_graphql::{OutputType, SimpleObject};
use rocket::{
    http::{ContentType, Status},
    response::{Responder, Result},
    serde::{
        json::{serde_json, Json},
        Serialize,
    },
    Request,
};

use crate::common::logger::Logger;
use crate::schemas::SchemaOut;

/// Generates a GraphQL response object and implementations.
macro_rules! generate_graphql_response_type {
    ($res_type:ident) => {
        #[derive(SimpleObject, Serialize)]
        #[serde(rename_all = "camelCase")]
        #[serde(crate = "rocket::serde")]
        pub struct $res_type<T>
        where
            T: SchemaOut + Debug + Serialize + Send + Sync + OutputType,
        {
            pub req_id: String,
            pub payload: T,
        }

        impl<T: SchemaOut + Debug + Serialize + Send + Sync + OutputType> Debug for $res_type<T> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(self).unwrap_or("Not available".into())
                )
            }
        }

        impl<T: SchemaOut + Debug + Serialize + Send + Sync + OutputType> $res_type<T> {
            /// Log the response.
            pub fn log(self, logger: &Logger) -> $res_type<T> {
                logger.info(&self);
                self
            }
        }
    };
}

// Contains a successful query requests payload and request ID.
generate_graphql_response_type!(SuccessQuery);

// Contains a successful mutation requests payload and request ID.
generate_graphql_response_type!(SuccessMutation);

pub type CatcherResponse = FailResponse<Fail<&'static str>>;

/// Return when request has failed.
#[derive(Debug)]
pub struct FailResponse<T>
where
    T: Debug + Serialize,
{
    pub error: Option<Json<T>>,
    pub status: Status,
}

impl<'r, T: Debug + Serialize> Responder<'r, 'r> for FailResponse<T> {
    fn respond_to(self, req: &'r Request) -> Result<'r> {
        rocket::Response::build_from(self.error.respond_to(req)?)
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

/// Contains a failed requests error message, code, and request ID.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct Fail<T>
where
    T: Serialize,
{
    pub req_id: String,
    pub error: T,
    pub code: u16,
}

impl<T: Serialize> Fail<T> {
    /// Log and return failed response.
    pub fn log_and_res(self, logger: &Logger, status: Status) -> FailResponse<Fail<T>> {
        logger.error(&self);
        FailResponse {
            error: Some(Json(self)),
            status,
        }
    }
}

impl<T: Serialize> Debug for Fail<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).unwrap_or("Not available".into())
        )
    }
}
