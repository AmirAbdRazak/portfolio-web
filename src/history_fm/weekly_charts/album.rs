use std::env;

use async_graphql::SimpleObject;
use futures::future::JoinAll;
use serde::Deserialize;
use tokio::task::JoinHandle;

use super::{get_chart_list, ArtistAttr, EntryAttr, WeeklyChartAttr};

#[derive(Deserialize, SimpleObject)]
struct AlbumEntry {
    name: String,
    playcount: String,
    #[serde(rename = "artist")]
    artist: ArtistAttr,
    #[serde(rename = "@attr")]
    attr: EntryAttr,
}

#[derive(Deserialize, SimpleObject)]
pub struct WeeklyAlbumChart {
    #[serde(rename = "@attr")]
    attr: WeeklyChartAttr,
    album: Vec<AlbumEntry>,
}

#[derive(Deserialize, SimpleObject)]
struct WeeklyAlbumChartResponse {
    weeklyalbumchart: WeeklyAlbumChart,
}

pub async fn get_album_chart_list<'a>(
    lastfm_username: &'a str,
) -> JoinAll<JoinHandle<WeeklyAlbumChart>> {
    let api_key = env::var("LASTFM_API_KEY").expect("LASTFM_API_KEY is not set");

    let available_chart_list = get_chart_list(&lastfm_username, &api_key)
        .await
        .expect("Error getting chart list");

    let results: Vec<JoinHandle<WeeklyAlbumChart>> = available_chart_list.into_iter().map(|chart| {
            let api_url = format!("http://ws.audioscrobbler.com/2.0/?method=user.getweeklyalbumchart&username={}&api_key={}&from={}&to={}&format=json&limit=200", lastfm_username, api_key, chart.from, chart.to);

            tokio::spawn(async move {
                surf::get(api_url)
                    .recv_json::<WeeklyAlbumChartResponse>()
                    .await
                    .expect("Error when calling surf API on weeklyalbumchart")
                    .weeklyalbumchart
            })
        }).collect();

    futures::future::join_all(results)
}
