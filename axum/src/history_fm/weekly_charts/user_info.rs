use dotenv::dotenv;
use serde::Deserialize;

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
pub struct UserInfo {
    pub name: String,
    pub image: Vec<ImageAttr>,
    pub playcount: String,
    pub artist_count: String,
    pub album_count: String,
    pub track_count: String,
    pub registered: RegisteredAttr,
}

#[derive(Deserialize)]
struct UserInfoResponse {
    user: UserInfo,
}

pub async fn get_user_info<'a>(username: &'a str, api_key: &'a str) -> UserInfo {
    dotenv().ok();
    let api_url = format!(
        "http://ws.audioscrobbler.com/2.0/?method=user.getinfo&user={}&api_key={}&format=json",
        username, api_key
    );

    surf::get(api_url)
        .recv_json::<UserInfoResponse>()
        .await
        .expect("Error when calling surf API on weeklychartlist")
        .user
}
