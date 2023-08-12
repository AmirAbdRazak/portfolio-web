use std::time::Instant;

use async_graphql::SimpleObject;
use futures::future::JoinAll;
use serde::Deserialize;
use tokio::task::JoinHandle;
use tracing::info;

use super::{get_chart_timestamp_list, ArtistAttr, EntryAttr, WeeklyChartAttr};

#[derive(Deserialize, SimpleObject)]
struct ChartEntry {
    name: String,
    playcount: String,
    #[serde(rename = "artist")]
    artist: Option<ArtistAttr>,
    #[serde(rename = "@attr")]
    attr: EntryAttr,
}

#[derive(Deserialize, SimpleObject)]
pub struct WeeklyChart {
    #[serde(rename = "@attr")]
    attr: WeeklyChartAttr,
    #[serde(alias = "artist", alias = "album", alias = "track")]
    chart: Vec<ChartEntry>,
}

#[derive(Deserialize, SimpleObject)]
pub struct WeeklyChartResponse {
    #[serde(
        alias = "weeklyartistchart",
        alias = "weeklyalbumchart",
        alias = "weeklytrackchart"
    )]
    pub weeklychart: WeeklyChart,
}

pub async fn get_weekly_chart_list<'a>(
    lastfm_username: &'a str,
    api_key: &'a str,
    chart_type: &'a str,
    registered_unixtime: u64,
) -> JoinAll<JoinHandle<WeeklyChartResponse>> {
    let start = Instant::now();
    let chart_list = get_chart_timestamp_list(registered_unixtime).await;
    info!(
        "Time Elapsed from fetching chart_list: {:?}",
        start.elapsed()
    );

    let results: Vec<JoinHandle<WeeklyChartResponse>> = chart_list.into_iter().map(|chart| {
            let api_url = format!("http://ws.audioscrobbler.com/2.0/?method=user.getweekly{}chart&username={}&api_key={}&from={}&to={}&format=json&limit=200",chart_type, lastfm_username, api_key, chart.from, chart.to);

            tokio::spawn(async move {
                surf::get(api_url)
                    .recv_json::<WeeklyChartResponse>()
                    .await
                    .expect("Error when calling surf API on weeklychart")
            })
        }).collect();

    futures::future::join_all(results)
}
