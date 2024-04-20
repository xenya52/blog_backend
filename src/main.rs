#[macro_use]
extern crate rocket;

mod api;
mod models;
mod postgresql;

#[launch]
fn server() -> _ {
    api::run()
}