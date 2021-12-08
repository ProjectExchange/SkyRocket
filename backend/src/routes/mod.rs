use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{Build, Rocket};
use rocket_okapi::mount_endpoints_and_merged_docs;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

mod addresses;
mod docs;
mod login;
mod offers;
mod users;

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct ErrorBody {
    pub error: String,
}

pub type ApiError = status::Custom<Json<ErrorBody>>;

pub type ApiResult<T> = std::result::Result<T, ApiError>;

pub fn error<Error>(err: Error, status: Status, message: &str) -> ApiError
where
    Error: ToString,
{
    eprintln! { "{}", err.to_string() };
    status::Custom(
        status,
        Json(ErrorBody {
            error: message.into(),
        }),
    )
}

/// This schema provides redirect url's for all configured OAuth providers. If a provider is not
/// configured, a null value is returned
#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct OAuthProviders {
    /// Redirect url for GitHub OAuth
    pub github: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct GitHubAccessTokenRequest<'a> {
    pub client_id: &'a str,
    pub client_secret: &'a str,
    pub code: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct GitHubAccessTokenResponse {
    pub access_token: String,
    pub scope: String,
    pub token_type: String,
}

pub fn init() -> Rocket<Build> {
    let mut rocket = rocket::build().attach(docs::stage());

    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();

    mount_endpoints_and_merged_docs! {
        rocket, "/v1".to_owned(), openapi_settings,
        "/users" => users::get_routes_and_docs(&openapi_settings),
        "/users" => addresses::get_routes_and_docs(&openapi_settings),
        "/offers" => offers::get_routes_and_docs(&openapi_settings),
        "/users/login" => login::get_routes_and_docs(&openapi_settings),
    };

    rocket
}
