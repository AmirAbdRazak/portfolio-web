pub mod artist;
use async_graphql::{Object, SimpleObject};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use surf;
use tracing::info;

use self::artist::{get_artist_chart_list, WeeklyArtistChart};

// Enum support is pretty eh for async graphql if the enum variants aren't of the same structure, on top of that I couldn't pass generic traits and stuff into types
// Hence this abonimation is necessary, for now, maybe I'm being dumb about this

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
