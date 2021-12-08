use crate::db::models::NewAddress;
use crate::db::models::{Address, AuthUser};
use crate::db::Db;
use crate::oso::{OsoAction, OsoState};
use crate::routes::{error, ApiResult};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_okapi::{
    okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
};

#[openapi(tag = "Addresses")]
#[post("/<id>/addresses", data = "<new_addr>")]
async fn create(
    oso: &OsoState,
    actor: AuthUser,
    db: Db,
    id: i32,
    new_addr: Json<NewAddress>,
) -> ApiResult<()> {
    if oso.is_allowed(actor, OsoAction::Create, Address::dummy_for_user(id)) {
        Address::save(&db, id, new_addr.clone()).await.map_or_else(
            |e| Err(error(e, Status::InternalServerError, "")),
            |_res| Ok(()),
        )
    } else {
        Err(error("", Status::Forbidden, "Forbidden"))
    }
}

#[openapi(tag = "Addresses")]
#[get("/<id>/addresses")]
async fn read(oso: &OsoState, actor: AuthUser, db: Db, id: i32) -> ApiResult<Json<Vec<Address>>> {
    if oso.is_allowed(actor, OsoAction::Read, Address::dummy_for_user(id)) {
        Ok(Json(Address::all_from_user(&db, id).await))
    } else {
        Err(error("", Status::Forbidden, "Forbidden"))
    }
}

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: read, create]
}
