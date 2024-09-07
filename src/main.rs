use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use std::env;

mod config;
mod middleware;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let server = HttpServer::new(move || {
        App::new().route(
            "/",
            web::get().to(|| async { HttpResponse::Ok().body("hello from rust") }),
        )
    });

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8082".to_string());

    println!("Starting server at http://{}:{}", host, port);

    server.bind(format!("{}:{}", host, port))?.run().await
}
