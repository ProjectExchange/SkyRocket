use crate::db::models::DbResult;
use crate::db::Db;
use crate::db::{schema::flights, schema::flights_offers};
use crate::routes::{error, ApiResult};
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use once_cell::sync::Lazy;
use regex::Regex;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, Deserialize, Serialize, DbEnum, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub enum Currency {
    Dollar,
    Euro,
}

/// Regex to validate the ICAO of a given flight
static RE_ICAO: Lazy<Regex> = Lazy::new(|| Regex::new(r"[A-Z]{4}$").unwrap());

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, Validate)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
#[validate(schema(function = "arrival_greater_departure"))]
pub struct NewFlight {
    #[validate(regex = "RE_ICAO")]
    pub departure_icao: String,
    /// Must be formatted like `2015-07-01 08:59:60 +0000`
    pub departure_time: DateTime<Utc>,
    #[validate(regex = "RE_ICAO")]
    pub arrival_icao: String,
    /// Must be formatted like `2015-07-01 08:59:60 +0000`
    pub arrival_time: DateTime<Utc>,
}

impl NewFlight {
    pub fn is_valid(&self) -> ApiResult<()> {
        self.validate()
            .map_err(|e| error(e.clone(), Status::BadRequest, &e.to_string()))
    }
}

/// Custom validator function to make sure arrival succeeds departure
fn arrival_greater_departure(flight: &NewFlight) -> Result<(), ValidationError> {
    if flight.departure_time.timestamp() < flight.arrival_time.timestamp() {
        Ok(())
    } else {
        Err(ValidationError::new(
            "Invalid flights: Arrival happens before departure",
        ))
    }
}

#[derive(Debug, Clone, Insertable, AsChangeset, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "flights"]
pub struct InsertableFlight {
    pub offer_id: i32,
    pub departure_icao: String,
    pub departure_time: NaiveDateTime,
    pub arrival_icao: String,
    pub arrival_time: NaiveDateTime,
}

impl InsertableFlight {
    pub fn new(new_flight: &NewFlight, offer_id: i32) -> Self {
        InsertableFlight {
            offer_id,
            departure_icao: new_flight.departure_icao.clone(),
            departure_time: new_flight.departure_time.naive_utc(),
            arrival_icao: new_flight.arrival_icao.clone(),
            arrival_time: new_flight.arrival_time.naive_utc(),
        }
    }
}

#[derive(
    Debug, Clone, Queryable, Identifiable, Associations, Deserialize, Serialize, JsonSchema,
)]
#[serde(crate = "rocket::serde")]
#[belongs_to(FlightOffer, foreign_key = "offer_id")]
#[table_name = "flights"]
pub struct Flight {
    id: i32,
    #[serde(rename = "offerId")]
    offer_id: i32,
    #[serde(rename = "departureIcao")]
    pub departure_icao: String,
    #[serde(rename = "departureTime")]
    pub departure_time: NaiveDateTime,
    #[serde(rename = "arrivalIcao")]
    pub arrival_icao: String,
    #[serde(rename = "arrivalTime")]
    pub arrival_time: NaiveDateTime,
}

impl Flight {
    pub async fn all_from_offer(db: &Db, offer_id: i32) -> Vec<Flight> {
        db.run(move |conn| Flight::belonging_to(&FlightOffer::dummy(offer_id)).load(conn))
            .await
            .unwrap_or_else(|_| Vec::new())
    }
}

#[derive(Debug, Clone, Insertable, Deserialize, Serialize, JsonSchema, Validate)]
#[serde(crate = "rocket::serde")]
#[table_name = "flights_offers"]
pub struct NewFlightOffer {
    #[validate(range(min = 1, max = 2000))]
    seats: i32,
    #[validate(range(min = 1, max = 99999))]
    price: f32,
    currency: Currency,
}

impl NewFlightOffer {
    pub fn is_valid(&self) -> ApiResult<()> {
        self.validate()
            .map_err(|e| error(e.clone(), Status::BadRequest, &e.to_string()))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Identifiable, Queryable, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[table_name = "flights_offers"]
pub struct FlightOffer {
    id: i32,
    seats: i32,
    price: f32,
    currency: Currency,
}

impl FlightOffer {
    pub fn dummy(id: i32) -> Self {
        FlightOffer {
            id,
            seats: 0,
            price: 0.0,
            currency: Currency::Euro,
        }
    }

    pub async fn get_all(db: &Db) -> Vec<FlightOffer> {
        db.run(move |conn| flights_offers::table.load(conn))
            .await
            .unwrap_or_else(|_| Vec::new())
    }

    pub async fn save(db: &Db, new_offer: NewFlightOffer) -> DbResult {
        db.run(move |conn| {
            diesel::insert_into(flights_offers::table)
                .values(&new_offer)
                .execute(conn)
        })
        .await
    }

    pub async fn save_flights(db: &Db, offer_id: i32, flights: Json<Vec<NewFlight>>) -> DbResult {
        let insertable = flights
            .iter()
            .map(|flight| InsertableFlight::new(flight, offer_id))
            .collect::<Vec<InsertableFlight>>();

        db.run(move |conn| {
            diesel::insert_into(flights::table)
                .values(&insertable)
                .execute(conn)
        })
        .await
    }

    pub async fn last_inserted(db: &Db) -> Option<Json<FlightOffer>> {
        db.run(move |conn| {
            flights_offers::table
                .order(flights_offers::id.desc())
                .first(conn)
        })
        .await
        .map(Json)
        .ok()
    }
}
