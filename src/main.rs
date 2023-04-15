#[macro_use]
extern crate rocket;

use std::env;

use rocket::fairing::AdHoc;
use rocket::{Build, Rocket};

use crate::fairing::fairings::cors;
use crate::route::catchers::catchers;
use crate::route::routes::routes;

mod common;
mod fairing;
mod model;
mod route;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes())
        .register("/", catchers())
        .attach(cors())
        .attach(AdHoc::on_liftoff("Liftoff Printer", |r| {
            Box::pin(async move {
                println!(
                    "🚀 We have liftoff from http://{}:{}",
                    env::var("ROCKET_ADDRESS").unwrap_or(r.config().address.to_string()),
                    env::var("ROCKET_PORT").unwrap_or(r.config().port.to_string())
                );
            })
        }))
}
