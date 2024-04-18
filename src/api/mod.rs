use rocket::{Build, Rocket};

pub mod models;
pub mod routes;

pub fn run() -> Rocket<Build> {
    rocket::build().mount(
        "/",
        routes![
            routes::send_all_users,
            routes::send_user,
            routes::send_user_param,
            routes::send_todo,
            routes::send_todo_param
        ],
    )
}
