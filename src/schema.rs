use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
};

use crate::history_fm::HistoryFMQuery;

#[derive(Default)]
struct AddQuery;

#[Object]
impl AddQuery {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    async fn history_fm(&self) -> HistoryFMQuery {
        HistoryFMQuery
    }

    // async fn portfolio(&self) -> PortfolioQuery {
    //     PortfolioQueri
    // }
}

type GraphQLSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

pub async fn graphql_handler(
    schema: Extension<GraphQLSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
