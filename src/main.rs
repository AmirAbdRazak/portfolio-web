use async_graphql::{
    http::GraphiQLSource, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use serde::Serialize;
use std::net::SocketAddr;
use tracing::info;

// async fn graphql_handler()
#[derive(Default)]
struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_user(&self, username: String) -> User {
        let id = 1337;

        info!("It got into get_user");

        User { id, username }
    }
}

#[derive(Default)]
struct AddQuery;

#[Object]
impl AddQuery {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

#[derive(Default)]
struct Query;

#[Object]
impl Query {
    async fn user(&self) -> UserQuery {
        UserQuery
    }

    async fn add(&self) -> AddQuery {
        AddQuery
    }
}

type GraphQLSchema = Schema<Query, EmptyMutation, EmptySubscription>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let schema = Schema::new(Query::default(), EmptyMutation, EmptySubscription);

    let app = Router::new()
        .route("/graphql", get(graphiql).post(graphql_handler))
        .layer(Extension(schema))
        .route("/", get(root));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::debug!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> String {
    format!("Hello, you have entered root")
}
async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

async fn graphql_handler(schema: Extension<GraphQLSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[derive(Serialize, SimpleObject)]
struct User {
    id: u64,
    username: String,
}
