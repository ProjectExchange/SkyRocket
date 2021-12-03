use crate::db::{schema::users_oauth_github, Db};
use crate::session;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::Request;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[table_name = "users_oauth_github"]
pub struct GitHubOAuthUser {
    pub user_id: i32,
    pub github_id: i32,
}

impl GitHubOAuthUser {
    pub async fn find_by_id(db: &Db, id: i32) -> Option<Json<Self>> {
        db.run(move |conn| users_oauth_github::table.find(id).first(conn))
            .await
            .map(Json)
            .ok()
    }

    pub async fn save(db: &Db, user: GitHubOAuthUser) -> Option<usize> {
        db.run(move |conn| {
            diesel::insert_into(users_oauth_github::table)
                .values(user)
                .execute(conn)
        })
        .await
        .ok()
    }
}

/// This schema is used to identify users that are registering a new GitHub OAuth connected account
#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct GithubOAuthRegistrar {
    pub github_id: i32,
}

impl GithubOAuthRegistrar {
    pub fn new(github_id: i32) -> Self {
        GithubOAuthRegistrar { github_id }
    }
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for GithubOAuthRegistrar {
    type Error = String;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(id) = session::get_github_id(request.cookies()).await {
            Outcome::Success(GithubOAuthRegistrar::new(id))
        } else {
            Outcome::Failure((Status::Forbidden, "Not logged in".to_owned()))
        }
    }
}

impl<'r> OpenApiFromRequest<'r> for GithubOAuthRegistrar {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}
