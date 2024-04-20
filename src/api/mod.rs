use rocket::{Build, Rocket};

pub mod models;
pub mod routes;

pub fn run() -> Rocket<Build> {
    rocket::build().mount("/", routes![routes::send_all_todos])
}