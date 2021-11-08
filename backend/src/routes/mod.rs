use rocket::fairing::AdHoc;
use rocket::response::Debug;

mod users;

pub type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Routes Stage", |rocket| async {
        rocket.attach(users::stage())
    })
}
