use actix_web::{web, App, HttpServer, HttpResponse};
use serde_json::json;

async fn hello() -> HttpResponse {
    HttpResponse::Ok().json(json!({"message": "Hello from Actix!"}))
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().json(json!({"status": "healthy"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on port 8080");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/health", web::get().to(health))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
