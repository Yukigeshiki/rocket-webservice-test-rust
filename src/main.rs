#[macro_use]
extern crate rocket;

use std::str::FromStr;

use rocket::{Build, Rocket};
use rocket_cors::{AllowedOrigins, CorsOptions};

use crate::catchers::supply_catchers;
use crate::routes::supply_routes;

mod catchers;
mod models;
mod routes;

#[launch]
fn rocket() -> Rocket<Build> {
    // Setup cors
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            ["Get", "Post"]
                .iter()
                .map(|s| FromStr::from_str(s).unwrap())
                .collect(),
        )
        .allow_credentials(true)
        .to_cors()
        .expect("Failed to setup cors configuration.");

    rocket::build()
        .mount("/", supply_routes())
        .register("/", supply_catchers())
        .attach(cors.clone())
        .manage(cors)
}
