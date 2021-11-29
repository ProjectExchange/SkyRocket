use super::schema::users;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
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
