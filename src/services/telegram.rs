use reqwest::{Body, Client};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TelegramDto<'a> {
    pub chat_id: &'a str,
    pub text: &'a String,
}
impl<'a> TelegramDto<'a> {
    pub fn new( text: &'a String) -> Self {
        TelegramDto {
            chat_id: "5669972257",
            text: text,
        }
    }

    pub fn serialize_body(self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    pub async fn send_message(
        bot_token: &'a String,
        body: impl Into<Body>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let client = Client::new();

        let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);

        client
            .post(url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
    }
}
