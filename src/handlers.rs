use std::vec;
use dotenv::dotenv;
use std::env;
//restapi test
use crate::{Todo, executions, todo_table_actions};

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

pub async fn get_post(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let url = get_database_url();
    let result = executions(url, todo_table_actions::Show).await;
    let mut database_content: Vec<Todo> = vec![];
    
    match result {
        Ok(stuff) => database_content = stuff,
        Err(e) => panic!("{}", e),
    }

    Ok(warp::reply::json(&database_content))
}
