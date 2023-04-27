//! Add your HTTP routes here.

use rocket::Route;

use crate::routes::get::get as get_route;
use crate::routes::post::post as post_route;

mod get;
mod post;

pub fn routes() -> Vec<Route> {
    routes![get_route, post_route]
}
