//! Add your app handlers here.

use async_graphql::{EmptySubscription, Schema};

use crate::handlers::mutation::Mutation;
use crate::handlers::query::Query;

pub mod mutation;
pub mod query;

pub type ProjectSchema = Schema<Query, Mutation, EmptySubscription>;
