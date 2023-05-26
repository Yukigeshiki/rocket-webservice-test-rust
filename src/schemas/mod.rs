//! Add your GraphQL schemas to this module.

pub mod id;
pub mod message;

pub use id::*;
pub use message::*;

pub trait SchemaOut {}

/// Implements the Debug trait for a model.
#[macro_export]
macro_rules! implement_schema_debug {
    ($schema:ty) => {
        impl Debug for $schema {
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
