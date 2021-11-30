use super::{error, ApiResult};
use crate::db::Db;
use crate::models::OAuthProviders;
use crate::models::User;
use crate::models::{GitHubAccessTokenRequest, GitHubAccessTokenResponse, GitHubOAuthUser};
use crate::session::create_session;
use crate::{http, CONFIG};
use diesel::prelude::*;
use rocket::http::uri::fmt::Query;
use rocket::http::uri::fmt::UriDisplay;
use rocket::http::CookieJar;
use rocket::http::Status;
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

/// Login endpoint for GitHub OAuth
///
/// The returned user variable has an id of null, iff the GitHub user hasn't
/// registered yet. If has user has registered previously, a valid id is returned.
/// In every other case (e.g. internal server error), nothing is returned.
///
/// # Arguments
///
/// * `code` - The OAuth code recevied by GitHub.
#[openapi(tag = "Login")]
#[post("/oauth/github?<code>")]
async fn login_github(db: Db, code: String, cookies: &CookieJar<'_>) -> ApiResult<Json<User>> {
    // validate token received from github
    let oauth_res = http::post::<GitHubAccessTokenResponse, GitHubAccessTokenRequest>(
        "https://github.com/login/oauth/access_token",
        &GitHubAccessTokenRequest {
            client_id: CONFIG
                .oauth_github_client_id
                .as_ref()
                .ok_or(error(Status::InternalServerError, ""))?,
            client_secret: CONFIG
                .oauth_github_client_secret
                .as_ref()
                .ok_or(error(Status::InternalServerError, ""))?,
            code,
        },
    )
    .await
    .map_err(|_e| error(Status::Unauthorized, "Failed to validate OAuth code"))?;

    // fetch github user data
    let user_res = http::get::<Value>("https://api.github.com/user", &oauth_res.access_token)
        .await
        .map_err(|_e| error(Status::Unauthorized, "Failed to validate OAuth code"))?;

    let github_id = user_res
        .get("id")
        .ok_or(error(Status::InternalServerError, ""))?
        .as_i64()
        .ok_or(error(Status::InternalServerError, ""))? as i32;

    // find user in db
    use crate::schema::users_oauth_github::dsl::users_oauth_github;
    let github_user = db
        .run(move |conn| {
            users_oauth_github
                .find(github_id)
                .first::<GitHubOAuthUser>(conn)
        })
        .await
        .map_err(|_e| error(Status::InternalServerError, ""))
        .ok();

    if github_user.is_some() {
        use crate::schema::users::dsl::{id, users};
        let user = db
            .run(move |conn| {
                users
                    .filter(id.eq(github_user.unwrap().user_id))
                    .first::<User>(conn)
            })
            .await
            .map_err(|_e| error(Status::InternalServerError, ""))?;
        create_session(cookies, user.clone()).await;
        Ok(Json(user))
    } else {
        let mut iter = user_res
            .get("name")
            .ok_or(error(Status::InternalServerError, ""))?
            .as_str()
            .ok_or(error(Status::InternalServerError, ""))?
            .splitn(2, ' ');
        Ok(Json(User {
            id: None,
            firstname: iter.next().unwrap().into(),
            lastname: iter.next().unwrap_or("").into(),
            email: user_res
                .get("email")
                .ok_or(error(Status::InternalServerError, ""))?
                .as_str()
                .ok_or(error(Status::InternalServerError, ""))?
                .into(),
        }))
    }
}

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: oauth_list, login_github]
}
