use async_graphql::SimpleObject;
use futures::future::JoinAll;
use serde::Deserialize;
use tokio::task::JoinHandle;

use super::{get_chart_list, ArtistAttr, EntryAttr, WeeklyChartAttr};

#[derive(Deserialize, SimpleObject)]
struct TrackEntry {
    name: String,
    playcount: String,
    artist: ArtistAttr,
    #[serde(rename = "@attr")]
    attr: EntryAttr,
}

#[derive(Deserialize, SimpleObject)]
pub struct WeeklyTrackChart {
    #[serde(rename = "@attr")]
    attr: WeeklyChartAttr,
    track: Vec<TrackEntry>,
}

#[derive(Deserialize, SimpleObject)]
struct WeeklyTrackChartResponse {
    weeklytrackchart: WeeklyTrackChart,
}

pub async fn get_track_chart_list<'a>(
    lastfm_username: &'a str,
    api_key: &'a str,
    registered_unixtime: u64,
) -> JoinAll<JoinHandle<WeeklyTrackChart>> {
    let available_chart_list = get_chart_list(registered_unixtime).await;

    let results: Vec<JoinHandle<WeeklyTrackChart>> = available_chart_list.into_iter().map(|chart| {
            let api_url = format!("http://ws.audioscrobbler.com/2.0/?method=user.getweeklytrackchart&username={}&api_key={}&from={}&to={}&format=json&limit=200", lastfm_username, api_key, chart.from, chart.to);

            tokio::spawn(async move {
                surf::get(api_url)
                    .recv_json::<WeeklyTrackChartResponse>()
                    .await
                    .expect("Error when calling surf API on weeklytrackchart")
                    .weeklytrackchart
            })
        }).collect();

    futures::future::join_all(results)
}
