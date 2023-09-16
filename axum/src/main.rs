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
use axum_server::tls_rustls::RustlsConfig;
use dotenv::dotenv;
use rcgen::generate_simple_self_signed;
use rustls::ServerConfig;
use schema::Query;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use std::{env, net::SocketAddr};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;

fn cors_layer() -> CorsLayer {
    CorsLayer::very_permissive()
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

    let subject_alt_names = vec![
        "localhost".to_string(),
        "amrrzk.fly.dev".to_string(),
        "axum-backend.internal".to_string(),
        "axum-backend.fly.dev".to_string(),
        "amirrazak.com".to_string(),
    ];

    let rcgen_cert = generate_simple_self_signed(subject_alt_names).unwrap();
    let cert = rustls::Certificate(rcgen_cert.serialize_der().expect("Couldn't serialize cert"));
    let private_key = rustls::PrivateKey(rcgen_cert.serialize_private_key_der());

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(vec![cert], private_key)
        .expect("Bad certificate in main");

    info!("Starting the server...");
    let addr = "[::]:8000".parse::<SocketAddr>().unwrap();
    tracing::debug!("Listening on {}", addr);

    axum_server::bind_rustls(addr, RustlsConfig::from_config(Arc::new(config)))
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn health_check() -> &'static str {
    "healthy"
}
