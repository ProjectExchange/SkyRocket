use crate::db::models::DbResult;
use crate::db::models::{AuthUser, User};
use crate::db::schema::addresses;
use crate::db::Db;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewAddress {
    country: String,
    postal_code: i32,
    town: String,
    street: String,
    house_number: i32,
}

#[derive(AsChangeset, Clone, Debug, Deserialize, Insertable, JsonSchema, Serialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "addresses"]
pub struct InsertableAddress {
    user_id: i32,
    country: String,
    postal_code: i32,
    town: String,
    street: String,
    house_number: i32,
}

impl InsertableAddress {
    pub fn new(addr: NewAddress, user_id: i32) -> Self {
        InsertableAddress {
            user_id,
            country: addr.country,
            postal_code: addr.postal_code,
            town: addr.town,
            street: addr.street,
            house_number: addr.house_number,
        }
    }
}

#[derive(Associations, Clone, Debug, Deserialize, Identifiable, JsonSchema, Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "addresses"]
pub struct Address {
    id: i32,
    user_id: i32,
    country: String,
    postal_code: i32,
    town: String,
    street: String,
    house_number: i32,
}

impl Address {
    pub async fn all_from_user(db: &Db, user: AuthUser) -> Vec<Address> {
        db.run(move |conn| Address::belonging_to(&User::dummy(user.id)).load(conn))
            .await
            .unwrap_or(Vec::new())
    }

    pub async fn save(db: &Db, user: AuthUser, new_addr: NewAddress) -> DbResult {
        db.run(move |conn| {
            diesel::insert_into(addresses::table)
                .values(&InsertableAddress::new(new_addr, user.id))
                .execute(conn)
        })
        .await
    }
}
