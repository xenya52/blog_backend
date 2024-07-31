mod model;
mod controller;

use actix_web::web::Data;
use actix_web::{App, HttpServer, Responder, get};
use mongodb::Client;
use dotenv::dotenv;

#[actix_web::get("/hello")]
async fn greet() -> impl Responder {
    format!("Hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = 8080;
    let host = "127.0.0.1";
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    let client = Client::with_uri_str(uri).await.expect("failed to connect");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(client.clone()))
            .service(greet)
    })
    .bind((host, port))?
    .workers(2)
    .run()
    .await
}

// current tutorial = https://www.youtube.com/watch?v=4Q7FAMydzOU