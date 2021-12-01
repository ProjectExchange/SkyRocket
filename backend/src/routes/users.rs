use crate::db::Db;
use crate::models::User;
use crate::routes::{error, ApiResult};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_okapi::{
    okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
};

#[openapi(tag = "Users")]
#[post("/", data = "<user>")]
async fn create(db: Db, user: Json<User>) -> ApiResult<Json<User>> {
    User::save_and_return(&db, user.into_inner())
        .await
        .ok_or(error(Status::InternalServerError, ""))
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
async fn read(db: Db, id: i32) -> ApiResult<Json<User>> {
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

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: list, read, create, delete]
}
