use rocket::serde::{Deserialize, Serialize};

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

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Fail<'r> {
    #[serde(rename = "reqId")]
    pub req_id: String,
    pub error: &'r str,
}

pub type Id = usize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct IdPayload {
    pub id: Id,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Message<'r> {
    message: &'r str,
}
