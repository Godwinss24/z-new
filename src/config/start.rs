use std::env;

use actix_web::{App, HttpResponse, HttpServer, web};

pub async fn start() -> Result<(), String> {

    let port: u16 = match env::var("PORT") {
        Ok(val) => match val.parse::<u16>() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("PORT environment variable is not a valid number, using default 4000");
                4000
            }
        },
        Err(_) => 4000, 
    };

    println!("running on port: {}", port);    

    HttpServer::new(|| {
        App::new().route(
            "/",
            web::get().to(async || HttpResponse::Ok().body("Helloooo")),
        )
    })
    .bind(("0.0.0.0", port))
    .map_err(|e| e.to_string())?
    .run()
    .await
    .map_err(|e| e.to_string())
}
