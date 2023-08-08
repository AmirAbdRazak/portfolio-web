use std::time::Instant;

use async_graphql::SimpleObject;
use futures::future::JoinAll;
use serde::Deserialize;
use tokio::task::JoinHandle;
use tracing::info;

use super::{get_chart_list, EntryAttr, WeeklyChartAttr};

#[derive(Deserialize, SimpleObject)]
struct ArtistEntry {
    name: String,
    playcount: String,
    #[serde(rename = "@attr")]
    attr: EntryAttr,
}

#[derive(Deserialize, SimpleObject)]
pub struct WeeklyArtistChart {
    #[serde(rename = "@attr")]
    attr: WeeklyChartAttr,
    artist: Vec<ArtistEntry>,
}

#[derive(Deserialize, SimpleObject)]
struct WeeklyArtistChartResponse {
    weeklyartistchart: WeeklyArtistChart,
}

pub async fn get_artist_chart_list<'a>(
    lastfm_username: &'a str,
    api_key: &'a str,
    registered_unixtime: u64,
) -> JoinAll<JoinHandle<WeeklyArtistChart>> {
    let start = Instant::now();
    let chart_list = get_chart_list(registered_unixtime).await;
    info!(
        "Time Elapsed from fetching chart_list: {:?}",
        start.elapsed()
    );

    let results: Vec<JoinHandle<WeeklyArtistChart>> = chart_list.into_iter().map(|chart| {
            let api_url = format!("http://ws.audioscrobbler.com/2.0/?method=user.getweeklyartistchart&username={}&api_key={}&from={}&to={}&format=json&limit=200", lastfm_username, api_key, chart.from, chart.to);

            tokio::spawn(async move {
                surf::get(api_url)
                    .recv_json::<WeeklyArtistChartResponse>()
                    .await
                    .expect("Error when calling surf API on weeklyartistchart")
                    .weeklyartistchart
            })
        }).collect();

    futures::future::join_all(results)
}
