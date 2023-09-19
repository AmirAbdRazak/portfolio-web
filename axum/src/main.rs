pub mod history_fm;
pub mod schema;

use crate::schema::graphql_handler;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::http::header::{
    ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
    ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_LENGTH, CONTENT_TYPE,
};
use axum::routing::post;
use axum::{extract::Extension, http::Method, routing::get, Router};
use dotenv::dotenv;
use schema::Query;
use sqlx::postgres::PgPoolOptions;
use std::{env, net::SocketAddr};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

fn cors_layer() -> CorsLayer {
    let allowed_origins = vec![
        "http://0.0.0.0:5173".parse().unwrap(),
        "http://localhost:5173".parse().unwrap(),
        "http://0.0.0.0:8000".parse().unwrap(),
        "https://amirrazak.com".parse().unwrap(),
        "https://amrrzk.fly.dev".parse().unwrap(),
    ];
    let allowed_methods = vec![Method::GET, Method::POST, Method::OPTIONS];

    let allowed_headers = vec![
        CONTENT_LENGTH,
        CONTENT_TYPE,
        ACCESS_CONTROL_ALLOW_ORIGIN,
        ACCESS_CONTROL_ALLOW_CREDENTIALS,
        ACCESS_CONTROL_ALLOW_HEADERS,
        ACCESS_CONTROL_ALLOW_METHODS,
    ];

    CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods(allowed_methods)
        .allow_headers(allowed_headers)
        .allow_credentials(true)
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt::init();
    info!("Its in axum server");
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap();

    info!("Connecting to database...");

    let pg_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    info!("Building the schema...");

    info!("Running migrations...");
    sqlx::migrate!("./migrations")
        .run(&pg_pool)
        .await
        .expect("Failed to migrate");
    info!("All migrations ran");

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(pg_pool)
        .finish();

    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .layer(Extension(schema))
        .layer(cors_layer())
        .layer(TraceLayer::new_for_http())
        .route("/health", get(health_check));

    info!("Starting the server...");
    let addr = "[::]:8000".parse::<SocketAddr>().unwrap();
    tracing::debug!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn health_check() -> &'static str {
    "healthy"
}
