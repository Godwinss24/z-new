pub mod config;
pub mod services;

use std::{env, time::Duration};

use config::start::start;
use services::scrape::scrape;

use crate::services::telegram::TelegramDto;

#[actix_web::main]
async fn main() -> Result<(), String> {
    tokio::spawn(async {
        let seconds: u64 = 900;
        loop {
            match scrape().await {
                Err(e) => {
                    let body = match TelegramDto::new(&e).serialize_body() {
                        Ok(x) => x,
                        Err(e) => {
                            println!("{}", e);
                            tokio::time::sleep(Duration::from_secs(seconds)).await;
                            continue;
                        }
                    };

                    let bot_token = match env::var("TELEGRAM_BOT_TOKEN") {
                        Ok(x) => x,
                        Err(e) => {
                            eprintln!("{}", e.to_string());
                            tokio::time::sleep(Duration::from_secs(seconds)).await;
                            continue;
                        }
                    };

                    match TelegramDto::send_message(&bot_token, body).await {
                        Err(e) => {
                            eprintln!(
                                "An error occured while sending telegram message {}",
                                e.to_string()
                            );
                            tokio::time::sleep(Duration::from_secs(seconds)).await;
                            continue;
                        }
                        _ => {}
                    };
                }
                _ => {}
            };
            tokio::time::sleep(Duration::from_secs(seconds)).await;
        }
    });
    start().await
}
