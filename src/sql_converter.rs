use tokio_postgres::{NoTls, Error};
use crate::Todo;

pub async fn todotable_to_jsonstring(database_url:String) -> String {
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;
    let mut todos: Vec<Todo> = Vec::new();
            let rows = client
                .query(
                    "SELECT * FROM todo",
                    &[],
                )
                .await;
                for row in rows {
                    let t = Todo {
                        id: row.get(0),
                        name: row.get(1),
                        description: row.get(2),
                    };
                    todos.push(t);
                }
                return todos.into_iter().map(|i| i.to_string()).collect::<String>();
                
    }