#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

use rocket_webservice_test::launch;

#[launch]
async fn rocket() -> Rocket<Build> {
    launch().await
}
