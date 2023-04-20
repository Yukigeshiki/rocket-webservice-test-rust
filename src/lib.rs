#[macro_use]
extern crate rocket;

use std::env;

use rocket::{
    fairing::AdHoc,
    {Build, Rocket},
};

use crate::common::{catchers, cors};
use crate::config::{AppConfig, APP_CONFIG};
use crate::repository::{db_services::Db, get_mongo_handle};
use crate::routes::routes;

mod common;
mod config;
mod models;
mod repository;
mod routes;

const MONGO_DB_NAME: &str = "test-cluster"; // your mongodb database name here

pub struct AppState {
    pub mongo: Db<mongodb::Database>,
}

pub async fn launch() -> Rocket<Build> {
    // stop the program if unable to load config
    APP_CONFIG
        .set(AppConfig::load())
        .expect("Unable to load config");

    // one or more Db objects can be created and managed by Rocket's state
    let mongo = Db::new(
        // stop the program if mongo fails to return a db handle
        get_mongo_handle(APP_CONFIG.get().unwrap().get_mongo_uri(), MONGO_DB_NAME)
            .await
            .expect("Failed to provide a MongoDB database handle during liftoff"),
    );

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
        .manage(AppState { mongo })
}
