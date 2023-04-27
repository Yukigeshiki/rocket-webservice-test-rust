use std::fmt::{Debug, Formatter};

use crate::implement_model_debug;
use rocket::serde::{json::serde_json, Serialize};

use crate::models::Model;

pub type Id = usize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct IdPayload {
    pub id: Id,
}

impl Model for IdPayload {}

implement_model_debug!(IdPayload);
