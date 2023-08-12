pub mod chart;
pub mod user_info;
use std::{env, time::Instant};

use async_graphql::{Context, Object, SimpleObject};
use chrono::{DateTime, Datelike, Duration, NaiveDateTime, Utc};
use dotenv::dotenv;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use tracing::info;

use self::{
    chart::{get_weekly_chart_list, WeeklyChart},
    user_info::get_user_info,
};

#[derive(Deserialize, SimpleObject)]
struct ArtistAttr {
    #[serde(rename = "#text")]
    text: String,
}

#[derive(Deserialize, SimpleObject)]
struct EntryAttr {
    rank: String,
}

#[derive(Deserialize, SimpleObject)]
struct WeeklyChartAttr {
    from: String,
    to: String,
}

#[derive(Debug, Deserialize)]
struct WeeklyChartEntry {
    from: i64,
    to: i64,
}

async fn get_chart_timestamp_list(start_timestamp: u64) -> Vec<WeeklyChartEntry> {
    let start = Instant::now();
    let current_naive = NaiveDateTime::from_timestamp_opt(start_timestamp as i64, 0)
        .expect("Failed to parse start timestamp");
    let current_datetime: DateTime<Utc> = DateTime::from_utc(current_naive, Utc);
    let days_to_sunday = current_datetime.weekday().num_days_from_sunday();

    let nearest_sunday = current_datetime - Duration::days(days_to_sunday as i64);
    let start_of_nearest_sunday = nearest_sunday
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .expect("Failed to parse nearest sunday")
        .timestamp();

    let mut current_timestamp = start_of_nearest_sunday + 43200;
    let end_timestamp = Utc::now().timestamp();
    let mut results: Vec<WeeklyChartEntry> = Vec::new();

    while current_timestamp < end_timestamp {
        results.push(WeeklyChartEntry {
            from: current_timestamp,
            to: current_timestamp + 604800,
        });

        current_timestamp += 604800;
    }

    info!(
        "Benchmarked time elapsed for calling inbuild chart list: {:?}",
        start.elapsed()
    );

    results
}

#[derive(Default)]
pub struct WeeklyChartsQuery;

#[Object]
impl WeeklyChartsQuery {
    async fn chart<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        lastfm_username: String,
        chart_type: String,
    ) -> Vec<WeeklyChart> {
        dotenv().ok();
        let artist_start = Instant::now();

        let api_key = env::var("LASTFM_API_KEY").expect("LASTFM_API_KEY is not set");
        let pool = ctx
            .data::<Pool<Postgres>>()
            .expect("Error connecting to Postgres pool connection");

        let start = Instant::now();
        let user_info = get_user_info(&lastfm_username, &api_key, pool).await;
        info!(
            "Time Elapsed from fetching user info: {:?}",
            start.elapsed()
        );

        let join_all_result = get_weekly_chart_list(
            &lastfm_username,
            &api_key,
            &chart_type,
            user_info.registered_unixtime,
        )
        .await
        .await;

        let weeklychart = join_all_result
            .into_iter()
            .filter_map(|result| {
                if let Ok(res) = result {
                    Some(res.weeklychart)
                } else {
                    None
                }
            })
            .collect();

        info!(
            "Time Elapsed from running chart fetch of type {}: {:?}",
            &chart_type,
            artist_start.elapsed()
        );

        weeklychart
    }
}
