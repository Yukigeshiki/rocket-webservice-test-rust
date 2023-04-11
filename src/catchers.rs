use rocket::serde::json::{serde_json::json, Value};
use rocket::Catcher;

pub fn supply_catchers() -> Vec<Catcher> {
    catchers![not_found]
}

#[catch(404)]
pub fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}
