use todo_database::{executions, routes, todo_table_actions, todotable_to_jsonstring};
use tokio_postgres::Error;


#[tokio::main]
async fn main() -> Result<(), Error> {
    //todotable_to_jsonstring(url);
    // let _ = executions(url, todo_table_actions::Insert).await;
    let routes = routes();

    println!("Server started at http://localhost:8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
    Ok(())
}