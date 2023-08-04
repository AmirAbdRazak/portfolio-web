use async_graphql::{Object, SimpleObject, Union};
use serde::Serialize;

#[derive(Serialize, SimpleObject)]
struct ArtistEntry {
    name: String,
    playcount: u32,
    rank: u8,
}

#[derive(Serialize, SimpleObject)]
struct AlbumEntry {
    name: String,
    artist_name: Vec<String>,
    playcount: u32,
    rank: u8,
}

#[derive(Serialize, SimpleObject)]
struct TrackEntry {
    name: String,
    album_name: String,
    artist_name: Vec<String>,
    playcount: u32,
    rank: u8,
}

#[derive(Union)]
enum WeeklyChartsEntry {
    Artist(ArtistEntry),
    Album(AlbumEntry),
    Track(TrackEntry),
}

#[derive(SimpleObject)]
struct WeeklyChartsEntryList {
    chart_from: u64,
    chart_to: u64,
    chart_list: Vec<WeeklyChartsEntry>,
}

#[derive(Default)]
pub struct HistoryFMQuery;

#[Object]
impl HistoryFMQuery {
    async fn get_weekly_charts(&self) -> WeeklyChartsQuery {
        WeeklyChartsQuery
    }
}

#[derive(Default)]
struct WeeklyChartsQuery;

#[Object]
impl WeeklyChartsQuery {
    async fn artist<'ctx>(
        &self,
        // ctx: &Context<'ctx>,
        // lastfm_username: String,
    ) -> Vec<WeeklyChartsEntryList> {
        let chart_list = vec![WeeklyChartsEntry::Artist(ArtistEntry {
            name: String::from("tutara peak"),
            playcount: 8,
            rank: 1,
        })];

        vec![WeeklyChartsEntryList {
            chart_from: 0,
            chart_to: 7,
            chart_list: chart_list,
        }]
    }
}
