use mongodb::{
    options::{ClientOptions, ServerApi, ServerApiVersion},
    {Client, Database},
};

pub mod db_services;

/// Returns a MongoDB database handle.
pub async fn get_mongo_handle(uri: String, db_name: &str) -> mongodb::error::Result<Database> {
    let mut client_options = ClientOptions::parse(uri).await?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options)?;

    Ok(client.database(db_name))
}
