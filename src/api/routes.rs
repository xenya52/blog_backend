use crate::models::{todo::Todo, user::User};
use rocket::serde::json::Json;
use std::{fs::File, path::Path};

use super::models::{SendTodoParamResult, SendUserParamResult};

#[get("/")]
pub fn send_all_users() -> Json<Vec<User>> {
    Json(get_users_json())
}

#[get("/<user_index>")]
pub fn send_user(user_index: String) -> Json<User> {
    let users_json = get_users_json();

    let parsed_index = user_index.parse::<usize>().unwrap();

    let user = users_json.get(parsed_index).expect("user_index is invalid");

    Json(user.clone())
}

#[get("/<user_index>/<param_name>")]
pub fn send_user_param(user_index: String, param_name: String) -> Json<SendUserParamResult> {
    let users_json = get_users_json();

    let parsed_index = user_index.parse::<usize>().unwrap();

    let user_memory_ref = users_json.get(parsed_index).expect("user_index is invalid");

    let user_clone = user_memory_ref.clone();

    Json(user_clone.get(&param_name).expect("invalid input"))
}

#[get("/<user_index>/todos/<todo_index>")]
pub fn send_todo(user_index: String, todo_index: String) -> Json<Todo> {
    let users_json = get_users_json();

    let parsed_user_index = user_index.parse::<usize>().unwrap();

    let user_memory_ref = users_json
        .get(parsed_user_index)
        .expect("user_index is invalid");

    let user_clone = user_memory_ref.clone();

    let todos = user_clone.todos;

    let parsed_todo_index = todo_index.parse::<usize>().unwrap();

    let todo_memory_ref = todos.get(parsed_todo_index).expect("todo indes is invalid");

    Json(todo_memory_ref.clone())
}

#[get("/<user_index>/todos/<todo_index>/<todo_param>")]
pub fn send_todo_param(
    user_index: String,
    todo_index: String,
    todo_param: String,
) -> Json<SendTodoParamResult> {
    let users_json = get_users_json();

    let parsed_user_index = user_index.parse::<usize>().unwrap();

    let user_memory_ref = users_json
        .get(parsed_user_index)
        .expect("user_index is invalid");

    let user_clone = user_memory_ref.clone();

    let todos = user_clone.todos;

    let parsed_todo_index = todo_index.parse::<usize>().unwrap();

    let todo_memory_ref = todos.get(parsed_todo_index).expect("todo indes is invalid");

    let todo_clone = todo_memory_ref.clone();

    Json(todo_clone.get(&todo_param).expect("todo param is invalid"))
}

fn get_users_json() -> Vec<User> {
    let current_path = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let file_path = format!("{current_path}/src/data.json", current_path = &current_path);

    let file_ref = Path::new(&file_path);

    let file_data = File::open(file_ref).unwrap();

    serde_json::from_reader(file_data).expect("Could not read or parse JSON file")
}
