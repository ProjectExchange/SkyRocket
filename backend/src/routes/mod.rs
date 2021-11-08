use rocket::{Build, Rocket};
use rocket::response::Debug;
use rocket_okapi::mount_endpoints_and_merged_docs;

mod docs;
mod users;

pub type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

pub fn init() -> Rocket<Build> {
    let mut rocket = rocket::build().attach(docs::stage());

    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();

    mount_endpoints_and_merged_docs! {
        rocket, "/v1".to_owned(), openapi_settings,
        "/users" => users::get_routes_and_docs(&openapi_settings),
    };

    rocket
}
