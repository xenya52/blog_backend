use todo_database::{executions, todo_table_actions, Todo};
use axum::{body::Body, http::{Response, StatusCode}, response::IntoResponse, routing::{get, post}, Json, Router};
use dotenv::dotenv;
use std::env;

use tower_http::cors::{Any, CorsLayer};
use http::Method;

fn get_database_url() -> String {
    
    dotenv().ok();
    let database_url = format!(
        "host={} user={} dbname={} password={}",
        env::var("DB_HOST").expect("DB_HOST must be set"),
        env::var("DB_USER").expect("DB_USER must be set"),
        env::var("DB_NAME").expect("DB_NAME must be set"),
        env::var("DB_PASSWORD").expect("DB_PASSWORD must be set"),
    );

    return database_url;
}

async fn get_todos_as_json() -> Json<Vec<Todo>> {
    let url = get_database_url();
    let result = executions(url, todo_table_actions::Show).await;
    
    let mut todos_response: Vec<Todo> = vec![];
    match result {
        Ok(stuff) => todos_response = stuff,
        Err(e) => panic!("{}", e),
    }
    Json(todos_response)
}

pub async fn create_todo_response() -> impl IntoResponse{
    Response::builder()
    .status(StatusCode::CREATED)
    .body(Body::from("todo created successfully"))
    .unwrap()
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        // allow any headers
        .allow_headers(AllowOrigin::any())
        // allow `POST` when accessing the resource
        .allow_methods(Any)
        // allow requests from below origins
        .allow_origin(Any);

    let app = Router::new()
    .route("/", get(get_todos_as_json))
    .layer(cors);

    println!("Running on http://localhost:8000");
    // Start Server
    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}