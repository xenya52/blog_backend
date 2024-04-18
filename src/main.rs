#[macro_use]
extern crate rocket;

mod api;
mod models;

#[launch]
fn server() -> _ {
    api::run()
}
