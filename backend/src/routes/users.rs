use crate::db::models::{GitHubOAuthUser, User};
use crate::db::Db;
use crate::routes::{error, ApiResult};
use crate::session;
use rocket::http::{CookieJar, Status};
use rocket::serde::json::Json;
use rocket_okapi::{
    okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
};

#[openapi(tag = "Users")]
#[post("/", data = "<user>")]
async fn create(db: Db, cookies: &CookieJar<'_>, user: Json<User>) -> ApiResult<Json<User>> {
    // retrieve GitHub id from users browser cookie
    let github_id = session::get_github_id(cookies).await.ok_or(error(
        Status::Unauthorized,
        "You are not using a supported OAuth provider",
    ))?;

    // save user value to db
    let user = User::save_and_return(&db, user.into_inner())
        .await
        .ok_or(error(Status::InternalServerError, ""))?;

    // link user to GitHub OAuth account
    GitHubOAuthUser::save(
        &db,
        GitHubOAuthUser {
            user_id: user.id.ok_or(error(Status::InternalServerError, ""))?,
            github_id,
        },
    )
    .await
    .ok_or(error(Status::InternalServerError, ""))?;

    session::revoke(cookies).await;
    session::set_user(cookies, user.clone()).await;
    Ok(user)
}

#[openapi(tag = "Users")]
#[get("/")]
async fn list(db: Db) -> ApiResult<Json<Vec<User>>> {
    User::get_all(&db)
        .await
        .ok_or(error(Status::InternalServerError, ""))
}

#[openapi(tag = "Users")]
#[get("/<id>")]
async fn read(_user: User, db: Db, id: i32) -> ApiResult<Json<User>> {
    User::find_by_id(&db, id)
        .await
        .ok_or(error(Status::NotFound, ""))
}

#[openapi(tag = "Users")]
#[delete("/<id>")]
async fn delete(db: Db, id: i32) -> ApiResult<()> {
    User::delete(&db, id)
        .await
        .ok_or(error(Status::NotFound, ""))
}

#[openapi(tag = "Users")]
#[get("/profile")]
async fn profile(cookies: &CookieJar<'_>) -> ApiResult<Json<User>> {
    session::get_user_from_session(cookies).await.map_or(
        Err(error(Status::Forbidden, "You are not logged in")),
        |u| Ok(Json(u)),
    )
}

#[openapi(tag = "Login")]
#[post("/logout")]
async fn logout(cookies: &CookieJar<'_>) -> ApiResult<()> {
    session::revoke(cookies)
        .await
        .ok_or(error(Status::Unauthorized, "No session to revoke"))
}

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: list, read, create, delete, profile, logout]
}
