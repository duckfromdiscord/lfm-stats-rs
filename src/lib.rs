pub mod json;

use crate::json::*;
use chrono::{DateTime, Utc};
use reqwest::Client;

#[derive(Debug, Clone)]
pub struct LastFMUser {
    pub name: String,
    pub age: u64,
    pub full_name: String,
    pub scrobbles: u64,
    pub artists: u64,
    pub scrobbling_since: DateTime<Utc>,
    pub gender: String,
}

pub fn get_client() -> Client {
    Client::builder().build().unwrap()
}

#[derive(Debug)]
pub enum LFMStatsError {
    SerdeError(serde_json::Error),
    ReqwestError(reqwest::Error),
}

pub async fn process_error<T: for<'de> serde::Deserialize<'de>>(
    response: Result<reqwest::Response, reqwest::Error>,
) -> Result<T, LFMStatsError> {
    match response {
        Ok(response) => {
            let json = response.text().await;
            match json {
                Ok(json) => match serde_json::from_str::<T>(&json) {
                    Ok(object) => Ok(object),
                    Err(error) => Err(LFMStatsError::SerdeError(error)),
                },
                Err(error) => Err(LFMStatsError::ReqwestError(error)),
            }
        }
        Err(error) => Err(LFMStatsError::ReqwestError(error)),
    }
}

pub async fn user_get_info_client(
    user: String,
    api_key: String,
    client: Client,
) -> Result<LastFMUser, LFMStatsError> {
    let response = client
        .get(format!(
            "https://ws.audioscrobbler.com/2.0/?method=user.getinfo&api_key={}&user={}&format=json",
            api_key, user
        ))
        .send()
        .await;
    match process_error::<UserInfo>(response).await {
        Ok(user_info) => Ok(LastFMUser {
            name: user_info.user.name,
            age: user_info.user.age.parse().unwrap(),
            full_name: user_info.user.realname,
            scrobbles: user_info.user.playcount.parse().unwrap(),
            artists: user_info.user.artist_count.parse().unwrap(),
            scrobbling_since: DateTime::from_timestamp(
                user_info.user.registered.text.unwrap().try_into().unwrap(),
                0,
            )
            .unwrap(),
            gender: user_info.user.gender,
        }),
        Err(error) => Err(error),
    }
}

pub async fn user_get_info(user: String, api_key: String) -> Result<LastFMUser, LFMStatsError> {
    user_get_info_client(user, api_key, get_client()).await
}