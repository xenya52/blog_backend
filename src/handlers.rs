
//restapi test

// pub async fn get_post(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
//     let url = get_database_url();
//     let result = executions(url, todo_table_actions::Show).await;
//     let mut database_content: Vec<Todo> = vec![];
    
//     match result {
//         Ok(stuff) => database_content = stuff,
//         Err(e) => panic!("{}", e),
//     }

//     Ok(warp::reply::json(&database_content))
// }
