//! Add your models to this module, along with a Debug implementation for clean and easy to analyse
//! log output. There is also a Model trait for common behavior shared among all models.

use crate::common::DateFormatter;

pub mod message;

pub use message::*;

pub type Id = String;

/// Can be implemented by each model.
pub trait Model {
    /// Adds a datetime to the created_at field. This is used when adding an item to the db.
    fn set_created_at(&mut self);

    /// Adds a Mongo Object ID.
    fn set_id(&mut self);
}

impl<M: Model> DateFormatter for M {}

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
