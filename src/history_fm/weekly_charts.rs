use async_graphql::{Object, SimpleObject};
use dotenv::dotenv;
use futures::future::{join, join_all, JoinAll};
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use surf;
use tokio::task::{JoinError, JoinHandle};
use tracing::info;

// Enum support is pretty eh for async graphql if the enum variants aren't of the same structure, on top of that I couldn't pass generic traits and stuff into types
// Hence this abonimation is necessary, for now, maybe I'm being dumb about this

#[derive(Deserialize, SimpleObject)]
struct ArtistEntryAttr {
    rank: String,
}
#[derive(Deserialize, SimpleObject)]
struct ArtistEntry {
    name: String,
    playcount: String,
    #[serde(rename = "@attr")]
    attr: ArtistEntryAttr,
}

#[derive(Deserialize, SimpleObject)]
struct WeeklyArtistChartAttr {
    from: String,
    to: String,
}

#[derive(Deserialize, SimpleObject)]
struct WeeklyArtistChart {
    #[serde(rename = "@attr")]
    attr: WeeklyArtistChartAttr,
    artist: Vec<ArtistEntry>,
}

#[derive(Deserialize, SimpleObject)]
struct WeeklyArtistChartResponse {
    weeklyartistchart: WeeklyArtistChart,
}

#[derive(Serialize, SimpleObject)]
struct AlbumEntry {
    name: String,
    artist_name: Vec<String>,
    playcount: u32,
    rank: u8,
}
#[derive(SimpleObject)]
struct AlbumEntryChart {
    chart_from: u64,
    chart_to: u64,
    chart_list: Vec<AlbumEntry>,
}

#[derive(Serialize, SimpleObject)]
struct TrackEntry {
    name: String,
    album_name: String,
    artist_name: Vec<String>,
    playcount: u32,
    rank: u8,
}
#[derive(SimpleObject)]
struct TrackEntryChart {
    chart_from: u64,
    chart_to: u64,
    chart_list: Vec<TrackEntry>,
}

#[derive(Debug, Deserialize)]
struct WeeklyChartEntry {
    #[serde(rename = "#text")]
    text: String,
    from: String,
    to: String,
    // Add other fields as needed
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
) -> Result<Vec<WeeklyChartEntry>, surf::Error> {
    dotenv().ok();
    let api_url = format!("http://ws.audioscrobbler.com/2.0/?method=user.getweeklychartlist&user={}&api_key={}&format=json", username, api_key);

    let WeeklyChartList { mut chart } = surf::get(api_url)
        .recv_json::<WeeklyChartListResponse>()
        .await
        .expect("Error when calling surf API on weeklychartlist")
        .weeklychartlist;

    chart.reverse();

    for c in &chart {
        info!("From {}, To {}", c.from, c.to);
    }

    Ok(chart)
}

async fn get_artist_chart_list<'a>(
    lastfm_username: &'a str,
) -> JoinAll<JoinHandle<WeeklyArtistChart>> {
    let api_key = env::var("LASTFM_API_KEY").expect("LASTFM_API_KEY is not set");

    let available_chart_list = get_chart_list(&lastfm_username, &api_key)
        .await
        .expect("Error getting chart list");

    let results: Vec<JoinHandle<WeeklyArtistChart>> = available_chart_list.into_iter().map(|chart| {
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

#[derive(Default)]
pub struct WeeklyChartsQuery;

#[Object]
impl WeeklyChartsQuery {
    async fn artist<'ctx>(
        &self,
        // ctx: &Context<'ctx>,
        lastfm_username: String,
    ) -> Vec<WeeklyArtistChart> {
        let join_all_result = get_artist_chart_list(&lastfm_username).await.await;

        join_all_result
            .into_iter()
            .filter_map(|result| result.ok())
            .collect()
    }

    async fn album<'ctx>(
        &self,
        // ctx: &Context<'ctx>,
        // lastfm_username: String,
    ) -> Vec<AlbumEntryChart> {
        todo!()
    }

    async fn track<'ctx>(
        &self,
        // ctx: &Context<'ctx>,
        // lastfm_username: String,
    ) -> Vec<TrackEntryChart> {
        todo!()
    }
}
