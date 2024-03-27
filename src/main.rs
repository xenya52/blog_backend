use todo_database::{todo_table_actions, executions, routes};
use tokio_postgres::Error;
use dotenv::dotenv;
use std::env;

fn get_database_url() -> String {
    // Load environment variables from .env file & construct the database connection string
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
#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = get_database_url();
    let _ = executions(url, todo_table_actions::Insert).await;

    let routes = routes();

    println!("Server started at http://localhost:8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
    Ok(())
}