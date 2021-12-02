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

use crate::db::models::User;
pub use config::CONFIG;
use oso::Oso;
use std::sync::{Arc, Mutex};

struct OsoState {
    oso: Arc<Mutex<Oso>>,
}

fn init_oso_state() -> Option<OsoState> {
    let mut oso = Oso::new();

    User::register_polar_class(&mut oso).ok()?;

    oso.load_files(vec!["security/users.polar"]).ok()?;

    Some(OsoState {
        oso: Arc::new(Mutex::new(oso)),
    })
}

// setting up rocket
#[launch]
fn rocket() -> _ {
    // load config and .env file, which is also required by the db module
    config::init();
    // init session storage with redis connection
    session::init();
    // initialize and start rocket server
    routes::init()
        .manage(init_oso_state().unwrap())
        .attach(db::stage())
}
