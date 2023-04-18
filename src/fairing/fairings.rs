//! Add your fairings here.

use rocket_cors::{AllowedOrigins, Cors, CorsOptions};
use std::str::FromStr;

pub fn cors() -> Cors {
    CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            ["Get", "Post"]
                .iter()
                .map(|s| FromStr::from_str(s).unwrap())
                .collect(),
        )
        .allow_credentials(true)
        .to_cors()
        .expect("Failed to setup cors configuration.")
}
