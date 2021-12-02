#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

pub mod db;
pub mod http;
pub mod session;

mod config;
mod routes;

pub use config::CONFIG;

// setting up rocket
#[launch]
fn rocket() -> _ {
    // load config and .env file, which is also required by the db module
    config::init();
    // init session storage with redis connection
    session::init();
    // initialize and start rocket server
    routes::init().attach(db::stage())
}
