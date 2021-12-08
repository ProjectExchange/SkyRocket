use crate::db::models::DbResult;
use crate::db::models::User;
use crate::db::schema::addresses;
use crate::db::Db;
use diesel::prelude::*;
use oso::PolarClass;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewAddress {
    country: String,
    #[serde(rename = "postalCode")]
    postal_code: i32,
    town: String,
    street: String,
    #[serde(rename = "houseNumber")]
    house_number: i32,
}

#[derive(AsChangeset, Clone, Debug, Deserialize, Insertable, Serialize)]
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

#[derive(
    Associations,
    Clone,
    Debug,
    Deserialize,
    PolarClass,
    Identifiable,
    JsonSchema,
    Queryable,
    Serialize,
)]
#[serde(crate = "rocket::serde")]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "addresses"]
pub struct Address {
    id: i32,
    #[polar(attribute)]
    #[serde(rename = "userId")]
    user_id: i32,
    country: String,
    #[serde(rename = "postalCode")]
    postal_code: i32,
    town: String,
    street: String,
    #[serde(rename = "houseNumber")]
    house_number: i32,
}

impl Address {
    /// Create a dummy address for a user with the given id. This function is
    /// used to construct an address object for use with oso policies.
    pub fn dummy_for_user(user_id: i32) -> Self {
        Address {
            id: 0,
            user_id,
            country: "".into(),
            postal_code: 0,
            town: "".into(),
            street: "".into(),
            house_number: 0,
        }
    }

    pub async fn all_from_user(db: &Db, user_id: i32) -> Vec<Address> {
        db.run(move |conn| Address::belonging_to(&User::dummy(user_id)).load(conn))
            .await
            .unwrap_or_else(|_| Vec::new())
    }

    pub async fn save(db: &Db, user_id: i32, new_addr: NewAddress) -> DbResult {
        db.run(move |conn| {
            diesel::insert_into(addresses::table)
                .values(&InsertableAddress::new(new_addr, user_id))
                .execute(conn)
        })
        .await
    }
}

pub(super) fn register_polar_classes(oso: &mut oso::Oso) -> oso::Result<()> {
    oso.register_class(Address::get_polar_class())
}
