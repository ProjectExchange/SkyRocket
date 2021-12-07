use crate::db::models::DbResult;
use crate::db::Db;
use crate::db::{schema::flights, schema::flights_offers};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Debug, Clone, Deserialize, Serialize, DbEnum, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub enum Currency {
    Dollar,
    Euro,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct NewFlight {
    departure_icao: String,
    departure_time: NaiveDateTime,
    arrival_icao: String,
    arrival_time: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, AsChangeset, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "flights"]
pub struct InsertableFlight {
    offer_id: i32,
    departure_icao: String,
    departure_time: NaiveDateTime,
    arrival_icao: String,
    arrival_time: NaiveDateTime,
}

impl InsertableFlight {
    pub fn new(new_flight: &NewFlight, offer_id: i32) -> Self {
        InsertableFlight {
            offer_id,
            departure_icao: new_flight.departure_icao.clone(),
            departure_time: new_flight.departure_time,
            arrival_icao: new_flight.arrival_icao.clone(),
            arrival_time: new_flight.arrival_time,
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
    offer_id: i32,
    departure_icao: String,
    departure_time: NaiveDateTime,
    arrival_icao: String,
    arrival_time: NaiveDateTime,
}

impl Flight {
    pub async fn all_from_offer(db: &Db, offer_id: i32) -> Vec<Flight> {
        db.run(move |conn| Flight::belonging_to(&FlightOffer::dummy(offer_id)).load(conn))
            .await
            .unwrap_or(Vec::new())
    }
}

#[derive(Debug, Clone, Insertable, Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[table_name = "flights_offers"]
pub struct NewFlightOffer {
    seats: i32,
    price: f32,
    currency: Currency,
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
            .unwrap_or(Vec::new())
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