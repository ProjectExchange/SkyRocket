use crate::CONFIG;
use crate::models::OAuthProviders;
use rocket::http::uri::fmt::Query;
use rocket::http::uri::fmt::UriDisplay;
use rocket::serde::json::Json;
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
async fn oauth_list<'a>() -> Json<OAuthProviders> {
    let github_url: Option<String>;

    match &CONFIG.oauth_github_client_id {
        &None => github_url = None,
        Some(value) => {
            let params = GitHubOAuth { client_id: value, scope: "user:email" };
            github_url = Some(format!(
                "https://github.com/login/oauth/authorize?{}",
                &params as &dyn UriDisplay<Query>
            ));
        },
    }

    Json(OAuthProviders {
        github: github_url,
    })
}

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: oauth_list]
}
