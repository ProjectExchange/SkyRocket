use crate::db::models::{
    AdminRole, AuthUser, Booking, GitHubOAuthUser, GithubOAuthRegistrar, NewUser, Role, Session,
    User,
};
use crate::db::Db;
use crate::oso::{OsoAction, OsoState};
use crate::routes::{error, ApiResult, UserAgent};
use crate::session;
use rocket::http::{CookieJar, Status};
use rocket::serde::json::Json;
use rocket_okapi::{
    okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
};

#[openapi(tag = "Users")]
#[post("/", data = "<new_user>")]
async fn create(
    actor: GithubOAuthRegistrar,
    db: Db,
    ua: UserAgent,
    cookies: &CookieJar<'_>,
    new_user: Json<NewUser>,
) -> ApiResult<Json<AuthUser>> {
    // validate user object
    new_user.clone().is_valid()?;

    // save user value to db
    let user = new_user
        .save_and_return(&db)
        .await
        .ok_or_else(|| error("", Status::InternalServerError, ""))?;

    // link user to GitHub OAuth account
    GitHubOAuthUser::save(
        &db,
        GitHubOAuthUser {
            user_id: user.id,
            github_id: actor.github_id,
        },
    )
    .await
    .ok_or_else(|| error("", Status::InternalServerError, ""))?;

    // add administrator rights if user was first user
    if user.is_first(&db).await {
        user.attach_role(&db, Role::Admin)
            .await
            .map_err(|e| error(e, Status::InternalServerError, ""))?;
    }

    // fetch permissions of the given user
    let auth_user = AuthUser::by_user_id(&db, user.id)
        .await
        .ok_or_else(|| error("", Status::InternalServerError, ""))?;

    // revoke GitHub ID session
    session::revoke(cookies).await;
    // save authenticated user session
    Session::save(&db, cookies, ua, auth_user.clone())
        .await
        .map_err(|e| {
            error(
                e,
                Status::InternalServerError,
                "Error saving session, please try again later",
            )
        })?;

    Ok(Json(auth_user))
}

#[openapi(tag = "Users")]
#[get("/")]
async fn list_for_admin(_r: AdminRole, db: Db) -> ApiResult<Json<Vec<User>>> {
    User::get_all(&db)
        .await
        .ok_or_else(|| error("", Status::InternalServerError, ""))
}

#[openapi(tag = "Users")]
#[get("/", rank = 2)]
async fn list_for_user(actor: AuthUser, db: Db) -> ApiResult<Json<Vec<User>>> {
    let user = User::find_by_id(&db, actor.id)
        .await
        .ok_or_else(|| error("", Status::InternalServerError, ""))?;
    Ok(Json(vec![user.into_inner()]))
}

#[openapi(tag = "Users")]
#[get("/<id>")]
async fn read(actor: AuthUser, oso: &OsoState, db: Db, id: i32) -> ApiResult<Json<User>> {
    let resource = User::find_by_id(&db, id)
        .await
        .ok_or_else(|| error("", Status::NotFound, ""))?;
    if oso.is_allowed(actor, OsoAction::Read, resource.clone()) {
        Ok(resource)
    } else {
        Err(error("", Status::Forbidden, "Forbidden"))
    }
}

#[openapi(tag = "Users")]
#[put("/<id>", data = "<new_user>")]
async fn update(
    actor: AuthUser,
    oso: &OsoState,
    db: Db,
    id: i32,
    new_user: Json<NewUser>,
) -> ApiResult<Json<User>> {
    if oso.is_allowed(actor, OsoAction::Update, User::dummy(id)) {
        // validate user object
        new_user.clone().is_valid()?;

        User::update_and_return(&db, id, new_user.clone())
            .await
            .ok_or_else(|| error("", Status::InternalServerError, ""))
    } else {
        Err(error("", Status::Forbidden, "Forbidden"))
    }
}

#[openapi(tag = "Users")]
#[delete("/<id>")]
async fn delete(actor: AuthUser, oso: &OsoState, db: Db, id: i32) -> ApiResult<()> {
    if oso.is_allowed(actor, OsoAction::Delete, User::dummy(id)) {
        User::delete(&db, id)
            .await
            .ok_or_else(|| error("", Status::NotFound, ""))
    } else {
        Err(error("", Status::Forbidden, "Forbidden"))
    }
}

#[openapi(tag = "Users")]
#[get("/profile")]
async fn profile(cookies: &CookieJar<'_>) -> ApiResult<Json<AuthUser>> {
    session::get_user_from_session(cookies).await.map_or(
        Err(error("", Status::Forbidden, "You are not logged in")),
        |u| Ok(Json(u)),
    )
}

#[openapi(tag = "Users")]
#[get("/<id>/bookings")]
async fn read_bookings(
    actor: AuthUser,
    oso: &OsoState,
    db: Db,
    id: i32,
) -> ApiResult<Json<Vec<Booking>>> {
    if oso.is_allowed(actor, OsoAction::Read, Booking::dummy(id)) {
        Ok(Json(Booking::all_from_user(&db, id).await))
    } else {
        Err(error("", Status::Forbidden, "Forbidden"))
    }
}

#[openapi(tag = "Login")]
#[post("/logout")]
async fn logout(db: Db, cookies: &CookieJar<'_>) -> ApiResult<()> {
    Session::delete_by_cookie(&db, cookies).await
}

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![
        settings: list_for_admin,
        list_for_user,
        read,
        create,
        update,
        delete,
        read_bookings,
        profile,
        logout
    ]
}
