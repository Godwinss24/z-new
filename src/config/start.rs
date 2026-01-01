use actix_web::{App, HttpResponse, HttpServer, web};

pub async fn start() -> Result<(), String> {
    HttpServer::new(|| {
        App::new().route(
            "/",
            web::get().to(async || HttpResponse::Ok().body("Helloooo")),
        )
    })
    .bind(("127.0.0.1", 3000))
    .map_err(|e| e.to_string())?
    .run()
    .await
    .map_err(|e| e.to_string())
}
