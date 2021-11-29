use crate::models::OAuthProviders;
use crate::models::{GitHubAccessTokenRequest, GitHubAccessTokenResponse};
use crate::{http, CONFIG};
use rocket::http::uri::fmt::Query;
use rocket::http::uri::fmt::UriDisplay;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::{openapi, openapi_get_routes_spec, settings::OpenApiSettings};

#[derive(UriDisplayQuery)]
struct GitHubOAuth<'a> {
    pub client_id: &'a str,
    pub scope: &'a str,
}

/// Returns a list of all configured OAuth providers and their corresponding redirect url's
#[openapi(tag = "Login")]
#[get("/oauth")]
async fn oauth_list() -> Json<OAuthProviders> {
    let github_url: Option<String>;

    match &CONFIG.oauth_github_client_id {
        &None => github_url = None,
        Some(value) => {
            let params = GitHubOAuth {
                client_id: value,
                scope: "user:email",
            };
            github_url = Some(format!(
                "https://github.com/login/oauth/authorize?{}",
                &params as &dyn UriDisplay<Query>
            ));
        }
    }

    Json(OAuthProviders { github: github_url })
}

/// Callback for GitHub OAuth
#[openapi(tag = "Login")]
#[get("/oauth/callback/github?<code>")]
async fn github_callback(code: String) -> Option<String> {
    let oauth_res = http::post::<GitHubAccessTokenResponse, GitHubAccessTokenRequest>(
        "https://github.com/login/oauth/access_token",
        &GitHubAccessTokenRequest {
            client_id: CONFIG.oauth_github_client_id.as_ref()?,
            client_secret: CONFIG.oauth_github_client_secret.as_ref()?,
            code,
        },
    )
    .await
    .ok()?;

    let user_res = http::get::<Value>("https://api.github.com/user", &oauth_res.access_token)
        .await
        .ok()?;

    let name = user_res.get("name")?;
    let email = user_res.get("email")?;
    Some(format! {"{} - {}", name, email})
}

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: oauth_list, github_callback]
}
