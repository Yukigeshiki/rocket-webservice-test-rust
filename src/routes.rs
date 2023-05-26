//! Add your HTTP routes here.

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use rocket::{
    response::content,
    {Route, State},
};

use crate::common::logger::Logger;
use crate::handlers::ProjectSchema;

pub fn routes() -> Vec<Route> {
    routes![graphql_query, graphql_request, graphql_playground]
}

#[get("/graphql?<query..>")]
async fn graphql_query(schema: &State<ProjectSchema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema.inner()).await
}

#[post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(
    mut logger: Logger,
    schema: &State<ProjectSchema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    logger.set_from("route::graphql_request");
    request.data(logger).execute(schema.inner()).await
}

#[get("/")]
async fn graphql_playground() -> content::RawHtml<String> {
    content::RawHtml(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
