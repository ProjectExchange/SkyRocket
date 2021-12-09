use crate::db::models::{FlightOfferWithOccupancy, User};
use crate::db::schema::bookings;
use crate::db::Db;
use crate::routes::{error, ApiResult};
use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Associations, Clone, Debug, Deserialize, JsonSchema, Insertable, Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "bookings"]
pub struct Booking {
    user_id: i32,
    offer_id: i32,
    seats: i32,
}

impl Booking {
    async fn save(db: &Db, booking: Booking) -> super::DbResult {
        db.run(move |conn| {
            diesel::insert_into(bookings::table)
                .values(&booking)
                .execute(conn)
        })
        .await
    }

    pub async fn create(db: &Db, user_id: i32, offer_id: i32, seats: i32) -> ApiResult<()> {
        let offer = FlightOfferWithOccupancy::from_offer_id(db, offer_id)
            .await
            .ok_or_else(|| error("", Status::NotFound, "Cannot find flight offer"))?;

        if seats < 1 || offer.seats - (offer.occupied as i32) > seats {
            return Err(error("", Status::BadRequest, "Bad number of seats"));
        }

        Booking::save(
            db,
            Booking {
                user_id,
                offer_id,
                seats,
            },
        )
        .await
        .map_or_else(
            |e| Err(error(e, Status::InternalServerError, "")),
            |_res| Ok(()),
        )
    }
}
