#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

pub mod db;
pub mod models;
pub mod schema;

mod routes;

// setting up rocket
#[launch]
fn rocket() -> _ {
    use dotenv::dotenv;
    dotenv().ok();

    rocket::build().attach(db::stage()).attach(routes::stage())
}
