use dotenv::dotenv;
use std::{env, future::Future};
use tokio_postgres::{NoTls, Error};
use std::fmt;

struct Todo {
    id: i32,
    name: String,
    description: String,
}
impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID: {}, Name: {}, Description: {}", self.id, self.name, self.description)
    }
}
enum todo_table_action {
    show,
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

async fn todo_table_action(database_url:String, action: todo_table_action) -> Result<(), Error> {
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
        todo_table_action::show => executable_string = "
        SELECT * FROM todo
        ",
        todo_table_action::insert => executable_string = "
        INSERT INTO public.todo
        (id, task_name, task_description)
        VALUES(nextval('todo_id_seq'::regclass), 'test_header', 'test_description');
        ",
        todo_table_action::init => executable_string = "
            CREATE TABLE IF NOT EXISTS todo (
            id SERIAL PRIMARY KEY,
            task_name VARCHAR NOT NULL,
            task_description TEXT
            )
        ",
        todo_table_action::drop => executable_string = "
        ",
        todo_table_action::merge => executable_string = "
        ",
        todo_table_action::update => executable_string =  "
        ",
        todo_table_action::delete => executable_string = "
        "
    }

    // Execute a statement to create a new table
    let result = client.execute(executable_string,
            &[],
        )
        .await;
    
    match result {
        Ok(_) => println!("Table action successfully executed."),
        Err(e) => return Err(e.into()), // Stellen Sie sicher, dass der Fehler in einen geeigneten Typ konvertiert wird.
    }        
    Ok(())
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
async fn insert_value_in_table(database_url:String) -> Result<(), Error> {
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
            "INSERT INTO public.todo
            (id, task_name, task_description)
            VALUES(nextval('todo_id_seq'::regclass), 'test_header', 'test_description');
            ",
            &[],
        )
        .await?;
    
    println!("Table dropped successfully");
    Ok(())
}
async fn delete_value_in_todo_table(database_url: String) -> Result<(), Error> {
    // Connect to the database
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Fetch and display the results
    let rows = client
        .query(
            "DELETE FROM todo
            WHERE id=1",
            &[],
        )
        .await?;

    for row in rows {
        let todo = Todo {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
        };

        // Print each Todo item
        println!("{}", todo);
    }

    Ok(())
}
async fn show_todo_table(database_url: String) -> Result<(), Error> {
    // Connect to the database
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Fetch and display the results
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

        // Print each Todo item
        println!("{}", todo);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = get_database_url();
    todo_table_action(url, todo_table_action::show);
    Ok(())
}