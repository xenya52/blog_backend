use crate::models::todo::Todo;
use crate::postgresql::executions::{sqlExecutions, todo_table_actions};
use rocket::serde::json::Json;

#[get("/")]
pub async fn send_all_todos() -> Json<Vec<Todo>> {
    Json(get_todos().await)
}

async fn get_todos() -> Vec<Todo> {
    let result = sqlExecutions(todo_table_actions::Show)
    .await
    .expect("Error retrieving todos");
    result
}