use rocket::serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

pub type Id = usize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct IdPayload {
    pub id: Id,
}

impl Debug for IdPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ \"id\": \"{}\" }}", self.id)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Message<'r> {
    message: &'r str,
}

impl Debug for Message<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ \"message\": \"{}\" }}", self.message)
    }
}
