//! Add your common application code to this module.

use chrono::Utc;
use chrono_tz::Africa::Johannesburg;

pub mod catchers;
pub mod fairings;
pub mod logger;
pub mod responses;

pub use catchers::*;
pub use fairings::*;
pub use logger::*;

/// A default datetime formatter.
pub trait DateFormatter {
    /// Returns a formatted ("%Y-%m-%d %H:%M:%S") String version of now in your timezone.
    fn now_and_format() -> String {
        Utc::now()
            .with_timezone(&Johannesburg) // change to your timezone here
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
    }
}
