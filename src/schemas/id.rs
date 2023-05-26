use std::fmt::{Debug, Formatter};

use crate::implement_schema_debug;
use async_graphql::{InputObject, SimpleObject};
use rocket::serde::{json::serde_json, Deserialize, Serialize};

use crate::schemas::SchemaOut;

pub type Id = usize;

/// Generate a schema object for an IdPayload type and implement Debug
macro_rules! generate_id_payload_schema_type {
    ($object:ty, $schema:ident) => {
        #[derive($object, Serialize, Deserialize)]
        #[serde(crate = "rocket::serde")]
        pub struct $schema {
            pub id: Id,
        }
        implement_schema_debug!($schema);
    };
}

// Generate Input and Simple Objects for IdPayloadIn/Out
generate_id_payload_schema_type!(InputObject, IdPayloadIn);
generate_id_payload_schema_type!(SimpleObject, IdPayloadOut);

impl SchemaOut for IdPayloadOut {}
