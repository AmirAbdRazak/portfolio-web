use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, SimpleObject};
use axum::{extract::State, routing::get, Router};
use serde::Serialize;
use serde_json;
use std::{net::SocketAddr, sync::Arc};
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

struct AppState {
    gql_res: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let schema = Schema::new(Query::default(), EmptyMutation, EmptySubscription);
    let res = schema
        .execute("{user {getUser(username: \"amrrzk\"){id, username}}}")
        .await;

    let data = res;
    let json = serde_json::to_string(&data);

    info!("{}", serde_json::to_string(&data).unwrap());

    let shared_state = Arc::new(AppState {
        gql_res: json.unwrap(),
    });

    let app = Router::new().route("/", get(root)).with_state(shared_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::debug!("Listening on {}", addr);
    info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root(State(state): State<Arc<AppState>>) -> String {
    let app_state = &Arc::as_ref(&state).gql_res;

    info!("It got into root");

    format!(
        "Hello, here is the result to your query: {}",
        app_state.to_owned()
    )
}

#[derive(Serialize, SimpleObject)]
struct User {
    id: u64,
    username: String,
}
