pub mod album;
pub mod artist;
pub mod track;
pub mod user_info;
use std::{env, time::Instant};

use async_graphql::{Context, Object, SimpleObject};
use chrono::{DateTime, Datelike, Duration, NaiveDateTime, Utc};
use dotenv::dotenv;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use tracing::info;

use self::{
    album::{get_album_chart_list, WeeklyAlbumChart},
    artist::{get_artist_chart_list, WeeklyArtistChart},
    track::{get_track_chart_list, WeeklyTrackChart},
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

async fn get_chart_list(start_timestamp: u64) -> Vec<WeeklyChartEntry> {
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
    async fn artist<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        lastfm_username: String,
    ) -> Vec<WeeklyArtistChart> {
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

        let join_all_result =
            get_artist_chart_list(&lastfm_username, &api_key, user_info.registered_unixtime)
                .await
                .await;

        let res = join_all_result
            .into_iter()
            .filter_map(|result| result.ok())
            .collect();

        info!(
            "Time Elapsed from running artist fetch: {:?}",
            artist_start.elapsed()
        );

        res
    }

    async fn album<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        lastfm_username: String,
    ) -> Vec<WeeklyAlbumChart> {
        let api_key = env::var("LASTFM_API_KEY").expect("LASTFM_API_KEY is not set");
        let pool = ctx
            .data::<Pool<Postgres>>()
            .expect("Error connecting to Postgres pool connection");

        let user_info = get_user_info(&lastfm_username, &api_key, pool).await;

        let join_all_result =
            get_album_chart_list(&lastfm_username, &api_key, user_info.registered_unixtime)
                .await
                .await;

        join_all_result
            .into_iter()
            .filter_map(|result| result.ok())
            .collect()
    }

    async fn track<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        lastfm_username: String,
    ) -> Vec<WeeklyTrackChart> {
        let api_key = env::var("LASTFM_API_KEY").expect("LASTFM_API_KEY is not set");
        let pool = ctx
            .data::<Pool<Postgres>>()
            .expect("Error connecting to Postgres pool connection");

        let user_info = get_user_info(&lastfm_username, &api_key, pool).await;

        let join_all_result =
            get_track_chart_list(&lastfm_username, &api_key, user_info.registered_unixtime)
                .await
                .await;

        join_all_result
            .into_iter()
            .filter_map(|result| result.ok())
            .collect()
    }
}
