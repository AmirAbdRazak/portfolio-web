pub mod album;
pub mod artist;
pub mod track;
use async_graphql::{Object, SimpleObject};
use dotenv::dotenv;
use serde::Deserialize;
use surf;
use tracing::info;

use self::{
    album::{get_album_chart_list, WeeklyAlbumChart},
    artist::{get_artist_chart_list, WeeklyArtistChart},
    track::{get_track_chart_list, WeeklyTrackChart},
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
        lastfm_username: String,
    ) -> Vec<WeeklyAlbumChart> {
        let join_all_result = get_album_chart_list(&lastfm_username).await.await;

        join_all_result
            .into_iter()
            .filter_map(|result| result.ok())
            .collect()
    }

    async fn track<'ctx>(
        &self,
        // ctx: &Context<'ctx>,
        lastfm_username: String,
    ) -> Vec<WeeklyTrackChart> {
        let join_all_result = get_track_chart_list(&lastfm_username).await.await;

        join_all_result
            .into_iter()
            .filter_map(|result| result.ok())
            .collect()
    }
}
