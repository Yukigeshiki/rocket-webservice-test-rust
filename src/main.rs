#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

mod routes;
mod models;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![routes::get, routes::post])
        .register("/", catchers![routes::not_found])
}
