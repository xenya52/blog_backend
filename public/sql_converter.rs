use std::slice::RSplit;

use tokio_postgres::{NoTls, Error};
use crate::Todo;

pub async fn todotable_to_jsonstring(database_url: String) -> Result<String, Box<dyn std::error::Error>> {
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let mut todos: Vec<Todo> = Vec::new();
    let rows = client
        .query("SELECT * FROM todo", &[])
        .await?;
    for row in rows {
        let t = Todo {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
        };
        todos.push(t);
    }
    let result = serde_json::to_string(&todos).map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;

    Ok(result)
}