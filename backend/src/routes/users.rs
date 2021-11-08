use super::Result;
use crate::db::Db;
use crate::models::User;
use crate::schema::users;
use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};

#[openapi(tag = "Users")]
#[post("/", data = "<user>")]
async fn create(db: Db, user: Json<User>) -> Result<Created<Json<User>>> {
    let user_value = user.clone();
    db.run(move |conn| {
        diesel::insert_into(users::table)
            .values(user_value)
            .execute(conn)
    })
    .await?;

    Ok(Created::new("/").body(user))
}

#[openapi(tag = "Users")]
#[get("/")]
async fn list(db: Db) -> Result<Json<Vec<User>>> {
    let ids: Vec<User> = db.run(move |conn| users::table.load::<User>(conn)).await?;

    Ok(Json(ids))
}

#[openapi(tag = "Users")]
#[get("/<id>")]
async fn read(db: Db, id: i32) -> Option<Json<User>> {
    db.run(move |conn| users::table.filter(users::id.eq(id)).first(conn))
        .await
        .map(Json)
        .ok()
}

#[openapi(tag = "Users")]
#[delete("/<id>")]
async fn delete(db: Db, id: i32) -> Result<Option<()>> {
    let affected = db
        .run(move |conn| {
            diesel::delete(users::table)
                .filter(users::id.eq(id))
                .execute(conn)
        })
        .await?;

    Ok((affected == 1).then(|| ()))
}

#[openapi(tag = "Users")]
#[delete("/")]
async fn destroy(db: Db) -> Result<()> {
    db.run(move |conn| diesel::delete(users::table).execute(conn))
        .await?;

    Ok(())
}

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: list, read, create, delete, destroy]
}
