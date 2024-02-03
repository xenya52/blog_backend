use dotenv::dotenv;
use std::env;
use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load environment variables from .env file
    dotenv().ok();

    // Construct the database connection string
    let database_url = format!(
        "host={} user={} dbname={} password={}",
        env::var("DB_HOST").expect("DB_HOST must be set"),
        env::var("DB_USER").expect("DB_USER must be set"),
        env::var("DB_NAME").expect("DB_NAME must be set"),
        env::var("DB_PASSWORD").expect("DB_PASSWORD must be set"),
    );

    // Connect to the database
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Execute a statement to create a new table
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS my_table (
                id SERIAL PRIMARY KEY,
                name VARCHAR NOT NULL,
                data JSONB
            )",
            &[],
        )
        .await?;

    println!("Table created successfully");

    Ok(())
}
