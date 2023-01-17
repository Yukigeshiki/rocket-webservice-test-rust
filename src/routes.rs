use rocket::serde::{json::{Json, Value, serde_json::json}};
use crate::models;

// The type to represent the ID of a message.
type Id = usize;

#[get("/<id>", format = "json")]
pub async fn get(id: Id) -> Value {
    json!({ "id": id })
}

#[post("/hello", format = "json", data = "<msg>")]
pub async fn post(msg: Json<models::Message<'_>>) -> Value {
    json!(*msg)
}

#[catch(404)]
pub fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}