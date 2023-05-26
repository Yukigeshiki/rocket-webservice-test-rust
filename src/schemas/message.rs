use std::fmt::{Debug, Formatter};

use crate::implement_schema_debug;
use async_graphql::{InputObject, SimpleObject};
use rocket::serde::{
    json::serde_json,
    {Deserialize, Serialize},
};

use crate::schemas::SchemaOut;

/// Generate a schema object for a Message type and implement Debug
macro_rules! generate_message_schema_type {
    ($object:ty, $schema:ident) => {
        #[derive($object, Serialize, Deserialize)]
        #[serde(crate = "rocket::serde")]
        pub struct $schema {
            pub message: String,
        }
        implement_schema_debug!($schema);
    };
}

// Generate Input and Simple Objects for MessageIn/Out
generate_message_schema_type!(InputObject, MessageIn);
generate_message_schema_type!(SimpleObject, MessageOut);

impl SchemaOut for MessageOut {}
