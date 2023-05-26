#[macro_use]
extern crate rocket;

use std::env;

use async_graphql::{EmptySubscription, Schema};
use rocket::{
    fairing::AdHoc,
    {Build, Rocket},
};

use crate::common::{catchers, cors};
use crate::config::{AppConfig, APP_CONFIG};
use crate::handlers::{mutation::Mutation, query::Query};
use crate::routes::routes;

mod common;
mod config;
mod handlers;
mod routes;
mod schemas;

pub async fn launch() -> Rocket<Build> {
    // stop the program if unable to load config
    APP_CONFIG
        .set(AppConfig::load())
        .expect("Unable to load config");
    let schema = Schema::build(Query, Mutation, EmptySubscription).finish();

    rocket::build()
        .manage(schema)
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
