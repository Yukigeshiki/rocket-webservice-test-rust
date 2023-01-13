#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
use rocket::serde::{Deserialize, Serialize, json::{Json, Value, serde_json::json}};

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

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![get, post])
        .register("/", catchers![not_found])
}
