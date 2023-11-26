use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Errorable<T> {
    #[serde(flatten)]
    inner: T,
    error: Option<String>,
    message: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct UserInfo {
    pub user: User,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct User {
    pub name: String,
    pub age: String,
    pub subscriber: String,
    pub realname: String,
    pub bootstrap: String,
    pub playcount: String,
    pub artist_count: String,
    pub playlists: String,
    pub track_count: String,
    pub album_count: String,
    pub image: Vec<Image>,
    pub registered: Registered,
    pub country: String,
    pub gender: String,
    pub url: String,
    #[serde(rename = "type")]
    // society if you could have a variable named `type` in Rust
    pub _type: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Image {
    pub size: Option<String>,
    #[serde(rename = "#text")]
    pub text: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Registered {
    pub unixtime: Option<String>,
    #[serde(rename = "#text")]
    pub text: Option<u64>,
}