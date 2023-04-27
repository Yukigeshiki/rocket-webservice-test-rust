use std::fmt::{Debug, Formatter};

use crate::implement_model_debug;
use rocket::serde::{json::serde_json, Deserialize, Serialize};

use crate::models::Model;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Message<'r> {
    message: &'r str,
}

impl Model for Message<'_> {}

implement_model_debug!(Message<'_>);
