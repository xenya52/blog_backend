use tokio_postgres::{NoTls, Error};
use crate::Todo;

pub enum todo_table_actions {
    Show,
    Init,
    Drop,
    Insert,
    Update,
    Delete,
    Merge,
}
pub async fn executions(database_url:String, action: todo_table_actions) -> Result<(), Error> {
    // Connect to the database
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;
    //Test conection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let mut executable_string: &str = "";
    match action {
        todo_table_actions::Show => executable_string = "
            SELECT * FROM todo 
        ",
        todo_table_actions::Insert => executable_string = "
            INSERT INTO public.todo
            (id, task_name, task_description)
            VALUES(nextval('todo_id_seq'::regclass), 'test_header', 'test_description');
        ",
        todo_table_actions::Init => executable_string = "
            CREATE TABLE IF NOT EXISTS todo (
            id SERIAL PRIMARY KEY,
            task_name VARCHAR NOT NULL,
            task_description TEXT
            )
        ",
        todo_table_actions::Drop => executable_string = "
            DROP TABLE todo
        ",
        todo_table_actions::Merge => executable_string = "
        ",
        todo_table_actions::Update => executable_string =  "
        ",
        todo_table_actions::Delete => executable_string = "
            DELETE FROM todo
            WHERE id=10
        "
    }
    println!("Execute table action...");
    let action_result = client.execute(
        executable_string,
        &[],
    )
    .await;
    match action_result {
        Ok(_) => {
            println!("-====<!Done!>====-");
            let rows = client
                .query(
                    "SELECT * FROM todo",
                    &[],
                )
                .await?;
                for row in rows {
                    let todo = Todo {
                        id: row.get(0),
                        name: row.get(1),
                        description: row.get(2),
                    };
                    println!("{}", todo);
                }
                println!("-====<!!!!!!>====-");
                Ok(())
            },
        Err(e) => {
            println!("{:?}", e);
            Err(e)
        }
    }
}