use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::Request;
use rocket::{Build, Rocket};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::mount_endpoints_and_merged_docs;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::request::OpenApiFromRequest;
use rocket_okapi::request::RequestHeaderInput;

mod addresses;
mod docs;
mod login;
mod offers;
mod sessions;
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

pub struct UserAgent(pub String);

#[rocket::async_trait]
impl<'a> FromRequest<'a> for UserAgent {
    type Error = String;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let ua = request.headers().get_one("user-agent");
        match ua {
            Some(ua) => {
                // check validity
                Outcome::Success(UserAgent(ua.to_string()))
            }
            None => Outcome::Failure((
                Status::Unauthorized,
                "User-Agent header must be defined to establish a session".into(),
            )),
        }
    }
}

impl<'r> OpenApiFromRequest<'r> for UserAgent {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}

pub fn init() -> Rocket<Build> {
    let mut rocket = rocket::build().attach(docs::stage());

    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();

    mount_endpoints_and_merged_docs! {
        rocket, "/v1".to_owned(), openapi_settings,
        "/users" => users::get_routes_and_docs(&openapi_settings),
        "/users" => addresses::get_routes_and_docs(&openapi_settings),
        "/users" => sessions::get_routes_and_docs(&openapi_settings),
        "/offers" => offers::get_routes_and_docs(&openapi_settings),
        "/users/login" => login::get_routes_and_docs(&openapi_settings),
    };

    rocket
}
