use dotenv::dotenv;
use serde::Deserialize;
use sqlx::Error;
use sqlx::{self, Pool, Postgres};
use tracing::info;

#[derive(Deserialize)]
pub struct RegisteredAttr {
    pub unixtime: String,
}

#[derive(Deserialize)]
pub struct ImageAttr {
    pub size: String,
    #[serde(rename = "#text")]
    pub text: String,
}
#[derive(Deserialize)]
pub struct LastFMUserInfo {
    pub name: String,
    pub image: Vec<ImageAttr>,
    pub playcount: String,
    pub artist_count: String,
    pub album_count: String,
    pub track_count: String,
    pub registered: RegisteredAttr,
}

pub struct UserInfo {
    pub username: String,
    pub image_url: String,
    pub playcount: u64,
    pub artist_count: u64,
    pub album_count: u64,
    pub track_count: u64,
    pub registered_unixtime: u64,
}

#[derive(Deserialize)]
struct LastFMUserInfoResponse {
    user: LastFMUserInfo,
}

pub async fn get_user_info<'a>(
    username: &'a str,
    api_key: &'a str,
    pool_result: &Result<Pool<Postgres>, Error>,
) -> UserInfo {
    info!("Inside getting user info from db...");

    let mut user_info_from_db = Err(Error::PoolClosed);

    if let Ok(pool) = pool_result {
        user_info_from_db = sqlx::query!(
                "SELECT username, image_url, playcount, artist_count, album_count, track_count, registered_unixtime FROM users where username ILIKE $1",
                &username
            )
            .fetch_one(pool)
            .await;
    }

    match user_info_from_db {
        Ok(user_info) => UserInfo {
            username: user_info.username,
            image_url: user_info
                .image_url
                .expect("Failed to fetch image url from user info"),
            playcount: user_info
                .playcount
                .expect("Failed to fetch playcount from user info") as u64,
            artist_count: user_info
                .artist_count
                .expect("Failed to fetch artist count from user info")
                as u64,
            album_count: user_info
                .album_count
                .expect("Failed to fetch album count from user info")
                as u64,
            track_count: user_info
                .album_count
                .expect("Failed to fetch track count from user info")
                as u64,
            registered_unixtime: user_info
                .registered_unixtime
                .expect("Failed to fetch unixtime from user info")
                as u64,
        },
        Err(_) => get_user_info_from_lastfm_api(username, api_key, pool_result).await,
    }
}

pub async fn get_user_info_from_lastfm_api<'a>(
    username: &'a str,
    api_key: &'a str,
    pool_result: &Result<Pool<Postgres>, Error>,
) -> UserInfo {
    dotenv().ok();

    let api_url = format!(
        "http://ws.audioscrobbler.com/2.0/?method=user.getinfo&user={}&api_key={}&format=json",
        username, api_key
    );

    let lastfm_user_info = surf::get(api_url)
        .recv_json::<LastFMUserInfoResponse>()
        .await
        .expect("Error when calling surf API on weeklychartlist")
        .user;

    let user_info = UserInfo {
        username: lastfm_user_info.name,
        image_url: lastfm_user_info
            .image
            .iter()
            .next()
            .expect("Failed to retrieve image url")
            .text
            .to_owned(),
        playcount: lastfm_user_info
            .playcount
            .parse::<u64>()
            .expect("Failed to parse playcount into u64"),
        artist_count: lastfm_user_info
            .artist_count
            .parse::<u64>()
            .expect("Failed to parse artist count into u64"),
        album_count: lastfm_user_info
            .album_count
            .parse::<u64>()
            .expect("Failed to parse album count into u64"),
        track_count: lastfm_user_info
            .track_count
            .parse::<u64>()
            .expect("Failed to parse track count into u64"),
        registered_unixtime: lastfm_user_info
            .registered
            .unixtime
            .parse::<u64>()
            .expect("Failed to parse unixtime into u64"),
    };

    if let Ok(pool) = pool_result {
        sqlx::query!("INSERT INTO users (username, image_url, playcount, artist_count, album_count, track_count, registered_unixtime) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            user_info.username,
            user_info.image_url,
            user_info.playcount as i64,
            user_info.artist_count as i64,
            user_info.album_count as i64,
            user_info.track_count as i64,
            user_info.registered_unixtime as i64).execute(pool).await.expect("Error inserting user info into db");
    }
    user_info
}
