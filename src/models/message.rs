use std::fmt::{Debug, Formatter};

use mongodb::bson::oid::ObjectId;
use rocket::serde::{
    json::serde_json,
    {Deserialize, Serialize},
};

use crate::common::DateFormatter;
use crate::implement_model_debug;
use crate::models::Model;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    /// Will be empty for both create and update requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub message: String,
    /// Will be empty for update requests.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Will never be empty - must be updated for every write request
    #[serde(rename = "updatedAt")]
    #[serde(default = "Message::now_and_format")]
    pub updated_at: String,
}

impl Model for Message {
    fn set_created_at(&mut self) {
        self.created_at = Some(Self::now_and_format());
    }

    fn set_id(&mut self) {
        self._id = Some(ObjectId::new());
    }
}

implement_model_debug!(Message);
