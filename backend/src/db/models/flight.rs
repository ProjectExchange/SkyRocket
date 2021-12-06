use crate::db::{schema::flights, schema::flights_offers};
use chrono::NaiveDateTime;
use diesel_derive_enum::DbEnum;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Debug, Clone, Deserialize, Serialize, DbEnum, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub enum Currency {
    Dollar,
    Euro,
}

#[derive(Debug, Clone, Insertable, Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[table_name = "flights"]
pub struct Flight {
    id: i32,
    offer_id: i32,
    departure_icao: String,
    departure_time: NaiveDateTime,
    arrival_icao: String,
    arrival_time: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct FlightOffer {
    id: i32,
    flights: Vec<Flight>,
    seats: i32,
    price: f32,
    currency: Currency,
}

#[derive(Debug, Clone, Insertable, Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[table_name = "flights_offers"]
pub struct InsertableFlightOffer {
    id: i32,
    seats: i32,
    price: f32,
    currency: Currency,
}
