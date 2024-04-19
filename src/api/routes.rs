use crate::models::todo::Todo;
use crate::postgresql::executions::{sqlExecutions, todo_table_actions};
use rocket::data;
use rocket::serde::json::Json;
use core::panicking::panic;
use std::{fs::File, path::Path};

use dotenv::dotenv;
use std::{env, result};

#[get("/")]
pub fn send_all_todos() -> Json<Vec<Todo>> {
    Json(get_todos_json())
}


fn get_database_url() -> String {

    dotenv().ok();
    let database_url = format!(
        "host={} user={} dbname={} password={}",
        env::var("DB_HOST").expect("DB_HOST must be set"),
        env::var("DB_USER").expect("DB_USER must be set"),
        env::var("DB_NAME").expect("DB_NAME must be set"),
        env::var("DB_PASSWORD").expect("DB_PASSWORD must be set"),
    );
    database_url
}

fn get_todos_json() -> Vec<Todo> {
    let current_path = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let result= sqlExecutions(get_database_url(), todo_table_actions::Show);
    match result.await {
        Ok(()) => {
            println!("test");
        }
    }
}
