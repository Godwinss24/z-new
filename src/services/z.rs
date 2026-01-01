#![allow(non_snake_case)]
use reqwest::Client;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct ZResponse {
    pub communities: Vec<Community>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Community {
    pub id: String,
    pub totalMembers: u64,
    pub twitterFollowers: u64,
    pub totalDiscordMembers: u64,
    pub name: String,
    pub createdAt: String,
    pub launchDate: Option<String>,
    pub website: Option<String>
}

pub async fn get_events() -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::new();

    let url = "https://api-v1.zealy.io/communities?category=new&page=0&limit=30";

    client
        .get(url)
        .header("Referer", "https://zealy.io/")
        .header("Origin", "https://zealy.io")
        .send()
        .await
}
