#[macro_use]
extern crate rocket;

use std::env;

use rocket::{Build, Rocket};

use crate::fairing::fairings::cors;
use crate::route::catchers::catchers;
use crate::route::routes::routes;

mod fairing;
mod model;
mod route;
mod util;

#[launch]
fn rocket() -> Rocket<Build> {
    let cors = cors();

    println!(
        "🚀 Rocket is launching from http://{}:{}",
        env::var("ROCKET_ADDRESS").unwrap_or("0.0.0.0".to_string()),
        env::var("ROCKET_PORT").unwrap_or(8080.to_string())
    );

    rocket::build()
        .mount("/", routes())
        .register("/", catchers())
        .attach(cors.clone())
        .manage(cors)
}
