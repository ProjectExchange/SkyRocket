use super::OfferFilter;
use crate::db::models::{
    AdminRole, AuthUser, Booking, Flight, FlightOffer, FlightOfferWithOccupancy, NewFlight,
    NewFlightOffer,
};
use crate::db::Db;
use crate::routes::{error, ApiResult};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_okapi::{
    okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
};

#[openapi(tag = "Flights")]
#[post("/", data = "<new_offer>")]
async fn create_offer(
    _r: AdminRole,
    db: Db,
    new_offer: Json<NewFlightOffer>,
) -> ApiResult<Json<FlightOffer>> {
    new_offer.is_valid()?;

    FlightOffer::save(&db, new_offer.clone())
        .await
        .map_err(|e| error(e, Status::InternalServerError, ""))?;

    FlightOffer::last_inserted(&db)
        .await
        .ok_or_else(|| error("", Status::InternalServerError, ""))
}

#[openapi(tag = "Flights")]
#[get("/?<filter..>")]
async fn read_offer(
    _actor: AuthUser,
    db: Db,
    filter: OfferFilter,
) -> ApiResult<Json<Vec<FlightOfferWithOccupancy>>> {
    Ok(Json(FlightOfferWithOccupancy::get_all(&db, filter).await))
}

#[openapi(tag = "Flights")]
#[get("/raw")]
async fn read_offer_raw(_r: AdminRole, db: Db) -> ApiResult<Json<Vec<FlightOffer>>> {
    Ok(Json(FlightOffer::get_all(&db).await))
}

#[openapi(tag = "Flights")]
#[post("/<id>/bookings?<seats>")]
async fn create_offer_booking(actor: AuthUser, db: Db, id: i32, seats: i32) -> ApiResult<()> {
    Booking::create(&db, actor.id, id, seats).await
}

#[openapi(tag = "Flights")]
#[get("/<id>/bookings")]
async fn read_offer_bookings(_r: AdminRole, db: Db, id: i32) -> ApiResult<Json<Vec<Booking>>> {
    Ok(Json(Booking::all_from_offer(&db, id).await))
}

#[openapi(tag = "Flights")]
#[get("/<id>/flights")]
async fn read_flights(_actor: AuthUser, db: Db, id: i32) -> ApiResult<Json<Vec<Flight>>> {
    Ok(Json(Flight::all_from_offer(&db, id).await))
}

#[openapi(tag = "Flights")]
#[post("/<id>/flights", data = "<new_flights>")]
async fn create_flights(
    _r: AdminRole,
    db: Db,
    id: i32,
    new_flights: Json<Vec<NewFlight>>,
) -> ApiResult<()> {
    let mut prev_time = 0;
    for flight in new_flights.clone().into_iter() {
        if flight.departure_time.clone().timestamp() > prev_time {
            prev_time = flight.arrival_time.clone().timestamp();
        } else {
            return Err(error(
                "Invalid array of flights",
                Status::BadRequest,
                "Departure of a flight must succeed arrival of previous flight",
            ));
        }
        flight.is_valid()?;
    }

    FlightOffer::save_flights(&db, id, new_flights)
        .await
        .map_or_else(
            |e| Err(error(e, Status::InternalServerError, "")),
            |_res| Ok(()),
        )
}

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![
        settings: create_offer,
        read_offer,
        read_offer_raw,
        create_offer_booking,
        read_offer_bookings,
        create_flights,
        read_flights
    ]
}
