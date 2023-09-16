use async_graphql::SimpleObject;
use futures::future::JoinAll;
use serde::Deserialize;
use tokio::task::JoinHandle;

#[derive(Deserialize, SimpleObject)]
pub struct EntryAttr {
    rank: String,
}

#[derive(Deserialize, SimpleObject)]
pub struct WeeklyChartAttr {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Deserialize)]
pub struct WeeklyChartEntry {
    pub from: i64,
    pub to: i64,
}
#[derive(Deserialize, SimpleObject)]
pub struct ArtistAttr {
    #[serde(rename = "#text")]
    pub text: String,
}
#[derive(Deserialize, SimpleObject)]
pub struct ChartEntry {
    pub name: String,
    pub playcount: String,
    #[serde(rename = "artist")]
    pub artist: Option<ArtistAttr>,
    #[serde(rename = "@attr")]
    pub attr: EntryAttr,
}

#[derive(Deserialize, SimpleObject)]
pub struct WeeklyChart {
    #[serde(rename = "@attr")]
    pub attr: WeeklyChartAttr,
    #[serde(alias = "artist", alias = "album", alias = "track")]
    pub chart: Vec<ChartEntry>,
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
    chart_list: &Vec<WeeklyChartEntry>,
) -> JoinAll<JoinHandle<WeeklyChartResponse>> {
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
