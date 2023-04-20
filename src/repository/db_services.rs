use std::fmt::Debug;

use rocket::serde::{Deserialize, Serialize};

/// A common interface for any db implementation (e.g. MongoDB, Postgresql, etc.). Having a
/// common interface can be handy for a few reasons; i.e. if you need to change out your db for a
/// different one at a later stage, or if you want to use multiple dbs in a project and want to
/// abstract away the specific calls for each one. This trait can be extended to include as many db
/// operations as required.
#[rocket::async_trait]
pub trait DatabaseService {
    // trait types can be extended as required
    type Error;
    type Filter: Send;
    type InsertOneResult;

    /// Fetch a single item of type I from the database filtering on filter.
    async fn read_one<I>(&self, filter: Self::Filter) -> Result<Option<I>, Self::Error>
    where
        for<'a> I: Deserialize<'a> + Send + Unpin + Sync + Debug;

    /// Write a single item of type I to the database.
    async fn write_one<I>(&self, item: &I) -> Result<Self::InsertOneResult, Self::Error>
    where
        for<'a> I: Deserialize<'a> + Serialize + Send + Unpin + Sync + Debug;
}

/// Holds a database handle or connection. This struct must implement the DatabaseService trait for
/// some database (e.g. MongoDB, Postgresql, etc.).
pub struct Db<D>
where
    D: 'static,
{
    database: D,
}

impl<D: 'static> Db<D> {
    pub fn new(d: D) -> Self {
        Self { database: d }
    }
}

const MONGO_COLLECTION: &str = "test-collection"; // your collection name here

/// DatabaseService implementation of Db for MongoDB. Options are None but can be included as needed.
#[rocket::async_trait]
impl DatabaseService for Db<mongodb::Database> {
    type Error = mongodb::error::Error;
    type Filter = mongodb::bson::Document;
    type InsertOneResult = mongodb::results::InsertOneResult;

    async fn read_one<I>(&self, filter: Self::Filter) -> Result<Option<I>, Self::Error>
    where
        for<'a> I: Deserialize<'a> + Send + Unpin + Sync + Debug,
    {
        self.database
            .collection::<I>(MONGO_COLLECTION)
            .find_one(Some(filter), None)
            .await
    }

    async fn write_one<I>(&self, item: &I) -> Result<Self::InsertOneResult, Self::Error>
    where
        for<'a> I: Deserialize<'a> + Serialize + Send + Unpin + Sync + Debug,
    {
        self.database
            .collection::<I>(MONGO_COLLECTION)
            .insert_one(item, None)
            .await
    }
}
