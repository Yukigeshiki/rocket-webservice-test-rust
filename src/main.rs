#[macro_use] extern crate rocket;

use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;

// The type to represent the ID of a message.
type Id = usize;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message<'r> {
    message: &'r str,
}

#[get("/<id>", format = "json")]
async fn get(id: Id) -> Value {
    json!({ "id": id })
}

#[post("/hello", format = "json", data = "<msg>")]
async fn post(msg: Json<Message<'_>>) -> Value {
    json!(*msg)
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![get, post])
        .register("/", catchers![not_found])
        .launch().await.expect("Panic");
}
