pub mod album;
pub mod artist;
pub mod track;
pub mod user_info;
use std::env;

use async_graphql::{Context, Object, SimpleObject};
use dotenv::dotenv;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use surf;

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
    from: String,
    to: String,
}

#[derive(Debug, Deserialize)]
struct WeeklyChartList {
    chart: Vec<WeeklyChartEntry>,
}

#[derive(Debug, Deserialize)]
struct WeeklyChartListResponse {
    weeklychartlist: WeeklyChartList,
}

async fn get_chart_list<'a>(
    username: &'a str,
    api_key: &'a str,
    registered_unixtime: u64,
) -> Result<Vec<WeeklyChartEntry>, surf::Error> {
    dotenv().ok();
    let api_url = format!("http://ws.audioscrobbler.com/2.0/?method=user.getweeklychartlist&user={}&api_key={}&format=json", username, api_key);

    let WeeklyChartList { chart } = surf::get(api_url)
        .recv_json::<WeeklyChartListResponse>()
        .await
        .expect("Error when calling surf API on weeklychartlist")
        .weeklychartlist;

    let filtered_chart = chart
        .into_iter()
        .filter(|chart_entry| {
            chart_entry
                .to
                .parse::<u64>()
                .expect("Failed to parse chart_entry['to'] into u64")
                > registered_unixtime
        })
        .collect();

    Ok(filtered_chart)
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
        let api_key = env::var("LASTFM_API_KEY").expect("LASTFM_API_KEY is not set");
        let pool = ctx
            .data::<Pool<Postgres>>()
            .expect("Error connecting to Postgres pool connection");

        let user_info = get_user_info(&lastfm_username, &api_key, pool).await;

        let join_all_result =
            get_artist_chart_list(&lastfm_username, &api_key, user_info.registered_unixtime)
                .await
                .await;

        join_all_result
            .into_iter()
            .filter_map(|result| result.ok())
            .collect()
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
