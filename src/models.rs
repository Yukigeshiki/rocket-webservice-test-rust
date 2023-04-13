use rocket::serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

/// Contains a successful requests payload and request ID.
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Success<T>
where
    T: Serialize,
{
    #[serde(rename = "reqId")]
    pub req_id: String,
    pub payload: T,
}

impl<T> Debug for Success<T>
where
    T: Debug + Serialize,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ \"req_id\": \"{}\", \"payload\": {:?} }}",
            self.req_id, self.payload
        )
    }
}

/// Contains a failed requests error message and request ID.
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Fail<'r> {
    #[serde(rename = "reqId")]
    pub req_id: String,
    pub error: &'r str,
}

impl Debug for Fail<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ \"req_id\": \"{}\", \"error\": \"{}\" }}",
            self.req_id, self.error
        )
    }
}

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
