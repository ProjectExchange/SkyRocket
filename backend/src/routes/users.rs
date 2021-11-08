use super::Result;
use crate::db::Db;
use crate::models::User;
use crate::schema::users;
use diesel::prelude::*;
use rocket::fairing::AdHoc;
use rocket::response::status::Created;
use rocket::serde::json::Json;

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

#[get("/")]
async fn list(db: Db) -> Result<Json<Vec<User>>> {
    let ids: Vec<User> = db.run(move |conn| users::table.load::<User>(conn)).await?;

    Ok(Json(ids))
}

#[get("/<id>")]
async fn read(db: Db, id: i32) -> Option<Json<User>> {
    db.run(move |conn| users::table.filter(users::id.eq(id)).first(conn))
        .await
        .map(Json)
        .ok()
}

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

#[delete("/")]
async fn destroy(db: Db) -> Result<()> {
    db.run(move |conn| diesel::delete(users::table).execute(conn))
        .await?;

    Ok(())
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("User Stage", |rocket| async {
        rocket.mount("/users", routes![list, read, create, delete, destroy])
    })
}
