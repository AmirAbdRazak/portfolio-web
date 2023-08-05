use async_graphql::{Object, SimpleObject};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use surf;
use tracing::info;

// Enum support is pretty eh for async graphql if the enum variants aren't of the same structure, on top of that I couldn't pass generic traits and stuff into types
// Hence this abonimation is necessary, for now, maybe I'm being dumb about this
#[derive(Serialize, SimpleObject)]
struct ArtistEntry {
    name: String,
    playcount: u32,
    rank: u8,
}
#[derive(SimpleObject)]
struct ArtistEntryChart {
    chart_from: u64,
    chart_to: u64,
    chart_list: Vec<ArtistEntry>,
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

async fn get_chart_list<'a>(username: &'a str) -> Result<Vec<WeeklyChartEntry>, surf::Error> {
    info!("Inside get chart list");
    dotenv().ok();
    let api_key = env::var("LASTFM_API_KEY").expect("LASTFM_API_KEY is not set");
    let api_url = String::from(format!("http://ws.audioscrobbler.com/2.0/?method=user.getweeklychartlist&user={}&api_key={}&format=json", username, api_key));

    let WeeklyChartList { chart } = surf::get(api_url)
        .recv_json::<WeeklyChartListResponse>()
        .await
        .expect("Error when calling surf API on weeklychartlist")
        .weeklychartlist;

    Ok(chart)
}

#[derive(Default)]
pub struct WeeklyChartsQuery;

#[Object]
impl WeeklyChartsQuery {
    async fn artist<'ctx>(
        &self,
        // ctx: &Context<'ctx>,
        // lastfm_username: String,
    ) -> Vec<ArtistEntryChart> {
        let available_chart_list = get_chart_list("ryzlegg")
            .await
            .expect("Error getting chart list");

        for chart in available_chart_list {
            info!("From: {}, To: {}", chart.from, chart.to);
        }

        let chart_list = vec![ArtistEntry {
            name: String::from("Tutara Peak"),
            playcount: 2540,
            rank: 1,
        }];

        vec![ArtistEntryChart {
            chart_from: 0,
            chart_to: 7,
            chart_list: chart_list,
        }]
    }

    async fn album<'ctx>(
        &self,
        // ctx: &Context<'ctx>,
        // lastfm_username: String,
    ) -> Vec<AlbumEntryChart> {
        let chart_list = vec![AlbumEntry {
            name: String::from("Bassika"),
            playcount: 945,
            rank: 1,
            artist_name: vec![String::from("Tutara Peak")],
        }];

        vec![AlbumEntryChart {
            chart_from: 0,
            chart_to: 7,
            chart_list: chart_list,
        }]
    }

    async fn track<'ctx>(
        &self,
        // ctx: &Context<'ctx>,
        // lastfm_username: String,
    ) -> Vec<TrackEntryChart> {
        let chart_list = vec![TrackEntry {
            name: String::from("Bassika: Act 1"),
            playcount: 407,
            rank: 1,
            artist_name: vec![String::from("Tutara Peak")],
            album_name: String::from("Bassika"),
        }];

        vec![TrackEntryChart {
            chart_from: 0,
            chart_to: 7,
            chart_list: chart_list,
        }]
    }
}
