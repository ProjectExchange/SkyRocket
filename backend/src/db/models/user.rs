use crate::db::{schema::users, Db};
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct User {
    pub id: Option<i32>,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

impl User {
    pub async fn get_all(db: &Db) -> Option<Json<Vec<User>>> {
        db.run(move |conn| users::table.load::<User>(conn))
            .await
            .map(Json)
            .ok()
    }

    pub async fn delete(db: &Db, id: i32) -> Option<()> {
        db.run(move |conn| {
            diesel::delete(users::table)
                .filter(users::id.eq(id))
                .execute(conn)
        })
        .await
        .ok()
        .map_or_else(|| Some(()), |_e| None)
    }

    pub async fn find_by_id(db: &Db, id: i32) -> Option<Json<Self>> {
        db.run(move |conn| users::table.find(id).first(conn))
            .await
            .map(Json)
            .ok()
    }

    pub async fn find_by_email(db: &Db, email: String) -> Option<Json<Self>> {
        db.run(move |conn| users::table.filter(users::email.eq(email)).first(conn))
            .await
            .map(Json)
            .ok()
    }

    pub async fn save(db: &Db, user: User) -> Option<usize> {
        db.run(move |conn| diesel::insert_into(users::table).values(user).execute(conn))
            .await
            .ok()
    }

    pub async fn save_and_return(db: &Db, user: User) -> Option<Json<Self>> {
        User::save(db, user.clone()).await?;
        User::find_by_email(db, user.email).await
    }
}
