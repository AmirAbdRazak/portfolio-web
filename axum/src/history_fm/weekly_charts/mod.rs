pub mod chart;
pub mod user_info;
use std::{collections::HashMap, env, time::Instant};

use async_graphql::{Context, Object, SimpleObject};
use chrono::{DateTime, Datelike, Duration, NaiveDateTime, Utc};
use dotenv::dotenv;
use serde::Serialize;
use sqlx::{Pool, Postgres};
use tracing::info;

use self::{
    chart::{get_weekly_chart_list, WeeklyChart, WeeklyChartEntry},
    user_info::get_user_info,
};

#[derive(Serialize)]
struct PlaycountData {
    playcount: Vec<u32>,
    prev_total: u32,
    last_iteration_update: u32,
}

#[derive(Serialize, SimpleObject)]
struct DatasetResult {
    chart_entry: String,
    playcount: Vec<u32>,
}

#[derive(Serialize, SimpleObject)]
struct ChartDataConfig {
    labels: Vec<u64>,
    datasets: Vec<DatasetResult>,
}

async fn get_chart_timestamp_list(start_timestamp: u64) -> Vec<WeeklyChartEntry> {
    let start = Instant::now();
    let current_naive = NaiveDateTime::from_timestamp_opt(start_timestamp as i64, 0)
        .expect("Failed to parse start timestamp");
    let current_datetime: DateTime<Utc> = DateTime::from_utc(current_naive, Utc);
    let days_to_sunday = current_datetime.weekday().num_days_from_sunday();

    let nearest_sunday = current_datetime - Duration::days(days_to_sunday as i64);
    let start_of_nearest_sunday = nearest_sunday
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .expect("Failed to parse nearest sunday")
        .timestamp();

    let mut current_timestamp = start_of_nearest_sunday + 43200;
    let end_timestamp = Utc::now().timestamp();
    let mut results: Vec<WeeklyChartEntry> = Vec::new();

    while current_timestamp < end_timestamp {
        results.push(WeeklyChartEntry {
            from: current_timestamp,
            to: current_timestamp + 604800,
        });

        current_timestamp += 604800;
    }

    info!(
        "Benchmarked time elapsed for calling inbuild chart list: {:?}",
        start.elapsed()
    );

    results
}

#[derive(Default)]
pub struct WeeklyChartsQuery;

#[Object]
impl WeeklyChartsQuery {
    async fn chart<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        lastfm_username: String,
        chart_type: String,
        limit: usize,
        offset: usize,
    ) -> ChartDataConfig {
        dotenv().ok();
        let fetch_start = Instant::now();

        let api_key = env::var("LASTFM_API_KEY").expect("LASTFM_API_KEY is not set");
        let pool = ctx
            .data::<Pool<Postgres>>()
            .expect("Error connecting to Postgres pool connection");

        let start = Instant::now();
        let user_info = get_user_info(&lastfm_username, &api_key, pool).await;
        info!(
            "Time Elapsed from fetching user info: {:?}",
            start.elapsed()
        );

        let join_all_result = get_weekly_chart_list(
            &lastfm_username,
            &api_key,
            &chart_type,
            user_info.registered_unixtime,
        )
        .await
        .await;

        let weekly_chart_list: Vec<WeeklyChart> = join_all_result
            .into_iter()
            .filter_map(|result| {
                if let Ok(res) = result {
                    Some(res.weeklychart)
                } else {
                    None
                }
            })
            .collect();

        info!(
            "Time Elapsed from running chart fetch of type {}: {:?}",
            &chart_type,
            fetch_start.elapsed()
        );

        let chart_start = Instant::now();

        let mut chart_dataset: HashMap<String, PlaycountData> = HashMap::new();

        let chart_len = weekly_chart_list.len() - 1;

        weekly_chart_list
            .into_iter()
            .enumerate()
            .for_each(|(iteration, weekly_charts)| {
                for chart in weekly_charts.chart {
                    let current_playcount = chart
                        .playcount
                        .parse::<u32>()
                        .expect("Expected u64 in attribute Chart Playcount");

                    chart_dataset
                        .entry(chart.name.clone())
                        .and_modify(|chart_map| {
                            let prev_total = chart_map.prev_total;
                            chart_map.prev_total += current_playcount;

                            if iteration > chart_map.last_iteration_update as usize + 1 {
                                let vec_fill =
                                    vec![
                                        prev_total;
                                        iteration - chart_map.last_iteration_update as usize - 1
                                    ];
                                chart_map.playcount.extend_from_slice(&vec_fill);
                            }

                            chart_map.playcount.push(current_playcount + prev_total);
                            chart_map.last_iteration_update = iteration as u32;
                        })
                        .or_insert({
                            let mut init_chart = vec![0; iteration];
                            init_chart.push(current_playcount);

                            PlaycountData {
                                playcount: init_chart,
                                prev_total: current_playcount,
                                last_iteration_update: iteration as u32,
                            }
                        });
                }
            });

        info!(
            "Time Elapsed from running chart dataset parsing : {:?}",
            chart_start.elapsed()
        );

        let mut playcount_list = Vec::new();

        for dataset in &chart_dataset {
            playcount_list.push(dataset.1.prev_total);
        }
        playcount_list.sort_by(|a, b| b.cmp(a));

        let benchmark_index;

        if limit + offset > playcount_list.len() {
            benchmark_index = playcount_list.len() - 1;
        } else {
            benchmark_index = limit + offset;
        }

        let upper_benchmark = playcount_list[offset];
        let lower_benchmark = playcount_list[benchmark_index];
        info!("offset: {}", offset);
        info!("limit: {}", limit);
        info!("benchmark index: {}", benchmark_index);
        info!("lower_benchmark: {}", lower_benchmark);
        info!("upper_benchmark: {}", upper_benchmark);

        ChartDataConfig {
            labels: get_chart_timestamp_list(user_info.registered_unixtime)
                .await
                .into_iter()
                .map(|t| t.to as u64)
                .collect(),
            datasets: chart_dataset
                .into_iter()
                .filter(|chart_entry| {
                    (chart_entry.1.prev_total > lower_benchmark)
                        & (chart_entry.1.prev_total <= upper_benchmark)
                })
                .map(|(chart_entry, mut playcount_data)| {
                    if playcount_data.last_iteration_update as usize <= chart_len {
                        let vec_fill = vec![
                            playcount_data.prev_total;
                            chart_len
                                - playcount_data.last_iteration_update as usize
                        ];
                        playcount_data.playcount.extend_from_slice(&vec_fill);
                    }
                    DatasetResult {
                        chart_entry,
                        playcount: playcount_data.playcount,
                    }
                })
                .collect(),
        }
    }
}
