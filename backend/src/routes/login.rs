use super::{error, ApiResult};
use super::{GitHubAccessTokenRequest, GitHubAccessTokenResponse, OAuthProviders};
use crate::db::models::Session;
use crate::db::models::{AuthUser, Gender, GitHubOAuthUser, NewUser};
use crate::db::Db;
use crate::routes::UserAgent;
use crate::session;
use crate::{http, CONFIG};
use chrono::NaiveDate;
use rocket::http::uri::fmt::Query;
use rocket::http::uri::fmt::UriDisplay;
use rocket::http::{CookieJar, Status};
use rocket::response::Responder;
use rocket::serde::json::{Json, Value};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::openapi3::{OpenApi, Responses};
use rocket_okapi::response::OpenApiResponderInner;
use rocket_okapi::util::add_schema_response;
use rocket_okapi::{openapi, openapi_get_routes_spec, settings::OpenApiSettings, Result};

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

/// Either returns an already registrated user, or some userdata that can be used to prefill forms
/// in case the user isn't registered yet.
#[derive(Responder)]
enum RegistratedOrNewUser {
    Registrated(Json<AuthUser>),
    New(Json<NewUser>),
}

#[rocket::async_trait]
impl OpenApiResponderInner for RegistratedOrNewUser {
    fn responses(gen: &mut OpenApiGenerator) -> Result<Responses> {
        let mut response = Responses::default();
        add_schema_response(
            &mut response,
            200,
            "application/json",
            gen.json_schema::<AuthUser>(),
        )?;
        add_schema_response(
            &mut response,
            200,
            "application/json",
            gen.json_schema::<NewUser>(),
        )?;
        Ok(response)
    }
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
async fn login_github(
    db: Db,
    code: String,
    cookies: &CookieJar<'_>,
    ua: UserAgent,
) -> ApiResult<RegistratedOrNewUser> {
    // validate token received from GitHub
    let oauth_res = http::post::<GitHubAccessTokenResponse, GitHubAccessTokenRequest>(
        "https://github.com/login/oauth/access_token",
        &GitHubAccessTokenRequest {
            client_id: CONFIG
                .oauth_github_client_id
                .as_ref()
                .ok_or_else(|| error("", Status::InternalServerError, ""))?,
            client_secret: CONFIG
                .oauth_github_client_secret
                .as_ref()
                .ok_or_else(|| error("", Status::InternalServerError, ""))?,
            code,
        },
    )
    .await
    .map_err(|e| error(e, Status::Unauthorized, "Failed to validate OAuth code"))?;

    // fetch GitHub user data
    let user_res = http::get::<Value>("https://api.github.com/user", &oauth_res.access_token)
        .await
        .map_err(|e| error(e, Status::Unauthorized, "Failed to validate OAuth code"))?;

    let github_id = user_res
        .get("id")
        .ok_or_else(|| error("", Status::InternalServerError, ""))?
        .as_i64()
        .ok_or_else(|| error("", Status::InternalServerError, ""))? as i32;

    if let Some(github_user) = GitHubOAuthUser::find_by_id(&db, github_id).await {
        let user = AuthUser::by_user_id(&db, github_user.user_id)
            .await
            .ok_or_else(|| error("", Status::InternalServerError, ""))?;
        Session::save(&db, cookies, ua, user.clone())
            .await
            .map_err(|e| {
                error(
                    e,
                    Status::InternalServerError,
                    "Error saving session, please try again later",
                )
            })?;
        Ok(RegistratedOrNewUser::Registrated(Json(user)))
    } else {
        session::set_github_id(cookies, github_id).await;
        let mut iter = user_res
            .get("name")
            .ok_or_else(|| error("", Status::InternalServerError, ""))?
            .as_str()
            .ok_or_else(|| error("", Status::InternalServerError, ""))?
            .splitn(2, ' ');
        Ok(RegistratedOrNewUser::New(Json(NewUser {
            firstname: iter.next().unwrap().into(),
            lastname: iter.next().unwrap_or("").into(),
            birthday: NaiveDate::from_yo(1970, 1),
            gender: Gender::Male,
            email: user_res
                .get("email")
                .ok_or_else(|| error("", Status::InternalServerError, ""))?
                .as_str()
                .unwrap_or("")
                .into(),
        })))
    }
}

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: oauth_list, login_github]
}
