//! Add your fairings here.

use std::str::FromStr;

use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};

const MAX_AGE_SECS: usize = 12 * 60 * 60;

pub fn cors() -> Cors {
    CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            ["Get", "Post"]
                .iter()
                .map(|s| FromStr::from_str(s).unwrap())
                .collect(),
        )
        .allowed_headers(AllowedHeaders::some(&[
            "Content-Type",
            "Content-Length",
            "Accept-Encoding",
            "Authorization",
            "Cache-Control",
        ]))
        .allow_credentials(true)
        .max_age(Some(MAX_AGE_SECS))
        .to_cors()
        .expect("Failed to setup cors configuration.")
}
