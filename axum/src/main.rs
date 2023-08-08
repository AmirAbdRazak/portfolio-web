pub mod history_fm;
pub mod schema;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{extract::Extension, routing::get, Router};
use dotenv::dotenv;
use schema::Query;
use sqlx::postgres::PgPoolOptions;
use std::{env, net::SocketAddr};
use tracing::info;

use crate::schema::{graphiql, graphql_handler};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt::init();
    info!("Its in axum server");
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap();

    info!("{}", database_url);

    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(pg_pool)
        .finish();

    let app = Router::new()
        .route("/graphql", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    info!("Starting the server...");
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::debug!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
