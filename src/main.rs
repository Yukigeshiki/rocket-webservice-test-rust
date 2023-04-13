#[macro_use]
extern crate rocket;

use std::env;
use std::str::FromStr;

use rocket::{Build, Rocket};
use rocket_cors::{AllowedOrigins, CorsOptions};

use crate::catchers::supply_catchers;
use crate::routes::supply_routes;

mod catchers;
mod models;
pub mod request;
pub mod response;
mod routes;
mod utils;

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

    println!(
        "🚀 Rocket is launching from http://{}:{}",
        env::var("ROCKET_ADDRESS").unwrap_or("0.0.0.0".to_string()),
        env::var("ROCKET_PORT").unwrap_or(8080.to_string())
    );

    rocket::build()
        .mount("/", supply_routes())
        .register("/", supply_catchers())
        .attach(cors.clone())
        .manage(cors)
}
