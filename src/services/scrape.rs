use dotenv::dotenv;
use std::{env, fs};

use crate::services::{
    telegram::TelegramDto,
    z::{Community, ZResponse, get_events},
};

pub async fn scrape() -> Result<(), String> {
    dotenv().ok();

    let response = match get_events().await {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Request error: {}", e);
            return Err(e.to_string());
        }
    };

    let text = match response.text().await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Response read error: {}", e);
            return Err(e.to_string());
        }
    };

    let z_response: ZResponse = match serde_json::from_str(&text) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Deserialize API response error: {}", e);
            return Err(e.to_string());
        }
    };

    match fs::read_to_string("z.json") {
        Ok(x) => {
            let parsed_file_content = match serde_json::from_str::<ZResponse>(x.as_str()) {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("Deserialize file content error: {}", e);
                    return Err(e.to_string());
                }
            };

            for i in z_response.communities.iter() {
                let w: Vec<&Community> = parsed_file_content
                    .communities
                    .iter()
                    .filter(|c| i.id == c.id)
                    .collect();

                if w.len() == 0 {
                    print!("different content detected");
                    match TelegramDto::new(&format!(
                        "A new event has been added: https://zealy.io/cw/{}/questboard",
                        i.name.to_lowercase()
                    ))
                    .serialize_body()
                    {
                        Ok(x) => {
                            let bot_token = match env::var("TELEGRAM_BOT_TOKEN") {
                                Ok(x) => x,
                                Err(e) => {
                                    eprintln!("{}", e.to_string());
                                    return Err(e.to_string());
                                }
                            };

                            match TelegramDto::send_message(&bot_token, x).await {
                                Err(e) => {
                                    eprintln!(
                                        "An error occured while sending telegram message {}",
                                        e.to_string()
                                    );
                                    return Err(e.to_string());
                                }
                                _ => {}
                            };

                            let z_response_string = match serde_json::to_string_pretty(&z_response)
                            {
                                Ok(x) => x,
                                Err(e) => {
                                    eprintln!(
                                        "An error occured while serializing z_response {}",
                                        e.to_string()
                                    );
                                    return Err(e.to_string());
                                }
                            };

                            match fs::write("z.json", z_response_string) {
                                Err(e) => {
                                    eprintln!(
                                        "An error occured while writing content to file: {}",
                                        e.to_string()
                                    );
                                    return Err(e.to_string());
                                }
                                _ => {}
                            }
                        }
                        Err(e) => {
                            eprintln!(
                                "An error occured while serializing telegram payload: {}",
                                e.to_string()
                            );
                            return Err(e.to_string());
                        }
                    };
                } else {
                    println!("same content")
                }
            }
            Ok(())
        }
        Err(_) => {
            let z_response_string = match serde_json::to_string_pretty(&z_response) {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("Error converting string to json: {}", e);
                    return Err(e.to_string());
                }
            };
            match fs::write("z.json", z_response_string) {
                Ok(_) => Ok(()),
                Err(e) => {
                    eprintln!("Deserialize API response error: {}", e);
                    return Err(e.to_string());
                }
            }
        }
    }
}
