pub mod weekly_charts;
use async_graphql::Object;
use weekly_charts::WeeklyChartsQuery;

#[derive(Default)]
pub struct HistoryFMQuery;
#[Object]
impl HistoryFMQuery {
    async fn get_weekly_charts(&self) -> WeeklyChartsQuery {
        WeeklyChartsQuery
    }
}
