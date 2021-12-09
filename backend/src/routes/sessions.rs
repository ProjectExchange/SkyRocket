use crate::db::models::{AuthUser, Session};
use crate::db::Db;
use crate::oso::{OsoAction, OsoState};
use crate::routes::{error, ApiResult};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_okapi::{
    okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
};

#[openapi(tag = "Sessions")]
#[get("/<user_id>/sessions")]
async fn read(
    oso: &OsoState,
    actor: AuthUser,
    db: Db,
    user_id: i32,
) -> ApiResult<Json<Vec<Session>>> {
    if oso.is_allowed(actor, OsoAction::Read, Session::dummy_for_user(user_id)) {
        Ok(Json(Session::all_from_user(&db, user_id).await))
    } else {
        Err(error("", Status::Forbidden, "Forbidden"))
    }
}

#[openapi(tag = "Sessions")]
#[delete("/<user_id>/sessions/<session_id>")]
async fn delete(
    oso: &OsoState,
    actor: AuthUser,
    db: Db,
    user_id: i32,
    session_id: i32,
) -> ApiResult<()> {
    if oso.is_allowed(actor, OsoAction::Delete, Session::dummy_for_user(user_id)) {
        Session::delete_by_id(&db, session_id).await
    } else {
        Err(error("", Status::Forbidden, "Forbidden"))
    }
}

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: read, delete]
}
