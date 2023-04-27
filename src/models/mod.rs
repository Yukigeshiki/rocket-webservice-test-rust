//! Add your models to this module, along with a Debug implementation for clean and easy to analyse
//! log output. There is also a Model trait for common behavior shared among all models.

pub mod id;
pub mod message;

pub use id::*;
pub use message::*;

pub trait Model {}

/// Implements the Debug trait for a model.
#[macro_export]
macro_rules! implement_model_debug {
    ($model:ty) => {
        impl Debug for $model {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(self).unwrap_or("Not available".into())
                )
            }
        }
    };
}
