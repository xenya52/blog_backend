use crate::models::todo::Todo;
use postgres::{Client, Error, NoTls};
use dotenv::dotenv;
use std::env;

//todo!!!
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

pub enum todo_table_actions {
    Show,
    Init,
    Drop,
    Insert,
    Update,
    Delete,
    Merge,
}
pub async fn sqlExecutions(action: todo_table_actions) -> Result<Vec<Todo>, Error> {
    // Connect to the database
    let mut client = Client::connect(&get_database_url(), NoTls)?;

    let mut executable_string: &str;
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
    let result = match client.query(executable_string, &[]) {
        Ok(stuff) => {
            println!("-====<!Done!>====-");
            let rows: Vec<postgres::Row> = client
                .query(
                    "SELECT * FROM todo",
                    &[],
                )?;
            let mut stuff: Vec<Todo> = vec![];
            for row in rows {
                let todo = Todo {
                    id: row.get(0),
                    name: row.get(1),
                    description: row.get(2),
                    checked: false
                };
                stuff.push(todo.clone());
                println!("{}", todo);
            }
            println!("-====<!!!!!!>====-");
            Ok(stuff)
            },
        Err(e) => {
            println!("{:?}", e);
            Err(e)
        }
    };

    result
}