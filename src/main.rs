use todo_database::{executions, routes, todo_table_actions, todotable_to_jsonstring};
use tokio_postgres::Error;


#[tokio::main]
async fn main() -> Result<(), Error> {
    // use insomnia !
    //todotable_to_jsonstring(url);
    // let _ = executions(url, todo_table_actions::Insert).await;
    // let routes = routes();
    // println!("Server started at http://localhost:8000");
    // warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
    // Ok(())
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default() // <-- Constructs the CORS middleware builder
                .allowed_origin("http://example.com") // <-- Specify the allowed origin
                .allowed_methods(vec!["GET", "POST"]) // <-- Specify the allowed methods
                .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT]) // <-- Specify the allowed headers
                .allowed_header(header::CONTENT_TYPE) // <-- You can specify individual headers
                .max_age(3600), // <-- Optional: specify the max cache duration for the preflight request
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
