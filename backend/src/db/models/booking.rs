use crate::db::models::DbResult;
use crate::db::models::User;
use crate::db::schema::bookings;
use crate::db::Db;
use diesel::prelude::*;
use oso::PolarClass;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Associations, Clone, Debug, Deserialize, JsonSchema, Insertable, Identifiable, Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
#[belongs_to(User, foreign_key = "user_id")]
#[primary_key("user_id", "offer_id")]
#[table_name = "bookings"]
pub struct Booking {
    user_id: i32,
    offer_id: i32,
    seats: i32,
}
