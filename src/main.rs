use dotenv::dotenv;
use std::env;
use tokio_postgres::{NoTls, Error};

enum todo_table_action {
    init,
    drop,
    insert,
    update,
    delete,
    merge,


}

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

async fn init_table(database_url:String) -> Result<(), Error> {
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
            "CREATE TABLE IF NOT EXISTS todo (
                id SERIAL PRIMARY KEY,
                task_name VARCHAR NOT NULL,
                task_description TEXT
            )",
            &[],
        )
        .await?;
    
    println!("Table created successfully");
    Ok(())
}

async fn drop_table(database_url:String) -> Result<(), Error> {
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
            "DROP TABLE todo
            ",
            &[],
        )
        .await?;
    
    println!("Table dropped successfully");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = get_database_url();

    drop_table(url).await?;

    Ok(())
}
