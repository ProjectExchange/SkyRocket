use crate::models::ErrorBody;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{Build, Rocket};
use rocket_okapi::mount_endpoints_and_merged_docs;

mod docs;
mod login;
mod users;

pub type ApiError = status::Custom<Json<ErrorBody>>;

pub type ApiResult<T> = std::result::Result<T, ApiError>;

pub fn error(status: Status, message: &str) -> ApiError {
    status::Custom(
        status,
        Json(ErrorBody {
            error: message.into(),
        }),
    )
}

pub fn init() -> Rocket<Build> {
    let mut rocket = rocket::build().attach(docs::stage());

    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();

    mount_endpoints_and_merged_docs! {
        rocket, "/v1".to_owned(), openapi_settings,
        "/users" => users::get_routes_and_docs(&openapi_settings),
        "/users/login" => login::get_routes_and_docs(&openapi_settings),
    };

    rocket
}
