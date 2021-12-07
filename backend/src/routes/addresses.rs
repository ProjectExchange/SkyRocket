use crate::db::models::NewAddress;
use crate::db::models::{Address, AuthUser};
use crate::db::Db;
use crate::routes::{error, ApiResult};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_okapi::{
    okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
};

#[openapi(tag = "Addresses")]
#[post("/", data = "<new_addr>")]
async fn create(actor: AuthUser, db: Db, new_addr: Json<NewAddress>) -> ApiResult<()> {
    Address::save(&db, actor, new_addr.clone())
        .await
        .map_or(Err(error(Status::InternalServerError, "")), |_res| Ok(()))
}

#[openapi(tag = "Addresses")]
#[get("/")]
async fn read(actor: AuthUser, db: Db) -> ApiResult<Json<Vec<Address>>> {
    Ok(Json(Address::all_from_user(&db, actor).await))
}

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: read, create]
}
