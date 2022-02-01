use async_graphql::extensions::apollo_persisted_queries::{
    ApolloPersistedQueries, LruCacheStorage,
};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use async_std::task;
use tide::{http::mime, Body, Response, StatusCode};

use std::env;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    task::block_on(run())
}

pub struct Query;

#[Object]
impl Query {
    async fn value(&self) -> i32 {
        100
    }
}

async fn run() -> Result<()> {
    let listen_addr = env::var("LISTEN_ADDR").unwrap_or_else(|_| "localhost:8000".to_owned());

    println!("Playground: http://{}", listen_addr);

    let mut app = tide::new();

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        // .data(data)
        .extension(ApolloPersistedQueries::new(LruCacheStorage::new(256)))
        .finish();

    app.at("/graphql").all(async_graphql_tide::graphql(schema));

    app.at("/").get(|_| async move {
        let mut resp = Response::new(StatusCode::Ok);
        resp.set_body(Body::from_string(playground_source(
            GraphQLPlaygroundConfig::new("/graphql"),
        )));
        resp.set_content_type(mime::HTML);
        Ok(resp)
    });

    app.listen(listen_addr).await?;

    Ok(())
}
