use rocket::serde::json::{serde_json::json, Json, Value};
use rocket::Route;

use crate::models;

// The type to represent the ID of a message.
type Id = usize;

pub fn supply_routes() -> Vec<Route> {
    routes![get, post]
}

#[get("/<id>", format = "json")]
pub async fn get(id: Id) -> Value {
    json!({ "id": id })
}

#[post("/hello", format = "json", data = "<msg>")]
pub async fn post(msg: Json<models::Message<'_>>) -> Value {
    json!(*msg)
}
