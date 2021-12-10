use crate::db::models::{FlightOffer, FlightOfferWithOccupancy, User};
use crate::db::schema::bookings;
use crate::db::Db;
use crate::routes::{error, ApiResult};
use diesel::prelude::*;
use oso::PolarClass;
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(
    Associations,
    Clone,
    Debug,
    Deserialize,
    JsonSchema,
    Insertable,
    Identifiable,
    PolarClass,
    Queryable,
    Serialize,
)]
#[serde(crate = "rocket::serde")]
#[belongs_to(User, foreign_key = "user_id")]
#[belongs_to(FlightOffer, foreign_key = "offer_id")]
#[primary_key(user_id, offer_id)]
#[table_name = "bookings"]
pub struct Booking {
    #[polar(attribute)]
    user_id: i32,
    offer_id: i32,
    seats: i32,
}

impl Booking {
    /// Create a dummy booking with a given user id. Used within oso policies
    pub fn dummy(user_id: i32) -> Self {
        Booking {
            user_id,
            offer_id: 0,
            seats: 0,
        }
    }

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

    pub async fn all_from_offer(db: &Db, offer_id: i32) -> Vec<Self> {
        db.run(move |conn| Booking::belonging_to(&FlightOffer::dummy(offer_id)).load(conn))
            .await
            .unwrap_or_else(|_| Vec::new())
    }

    pub async fn all_from_user(db: &Db, user_id: i32) -> Vec<Booking> {
        db.run(move |conn| Booking::belonging_to(&User::dummy(user_id)).load(conn))
            .await
            .unwrap_or_else(|_| Vec::new())
    }
}

pub(super) fn register_polar_classes(oso: &mut oso::Oso) -> oso::Result<()> {
    oso.register_class(Booking::get_polar_class())
}
