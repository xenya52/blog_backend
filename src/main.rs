use todo_database::{executions, todo_table_actions, Todo};
use axum::{body::Body, http::{Response, StatusCode, HeaderMap}, response::IntoResponse, routing::{get, post}, Json, Router};
use dotenv::dotenv;
use std::env;

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
    .header("Access-Control-Allow-Origin", "*")
    .body(Body::from("todo created successfully"))
    .unwrap()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(get_todos_as_json));

    println!("Running on http://localhost:8000");
    // Start Server
    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}