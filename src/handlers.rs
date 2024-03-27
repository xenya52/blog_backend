//restapi test
use crate::Todo;

// A function to handle GET requests at /posts/{id}
pub async fn get_post(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    // For simplicity, let's say we are returning a static post
    let post = Todo {
        id,
        name: String::from("Hello, Warp!"),
        description: String::from("This is a post about Warp."),
    };
    Ok(warp::reply::json(&post))
}