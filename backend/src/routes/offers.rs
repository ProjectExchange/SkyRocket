use crate::db::models::{AdminRole, AuthUser, Flight, FlightOffer, NewFlight, NewFlightOffer};
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
    FlightOffer::save(&db, new_offer.clone())
        .await
        .map_err(|_e| error(Status::InternalServerError, ""))?;

    FlightOffer::last_inserted(&db)
        .await
        .ok_or(error(Status::InternalServerError, ""))
}

#[openapi(tag = "Flights")]
#[get("/")]
async fn read_offer(_actor: AuthUser, db: Db) -> ApiResult<Json<Vec<FlightOffer>>> {
    Ok(Json(FlightOffer::get_all(&db).await))
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
    FlightOffer::save_flights(&db, id, new_flights)
        .await
        .map_or(Err(error(Status::InternalServerError, "")), |_res| Ok(()))
}

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![
        settings: create_offer,
        read_offer,
        create_flights,
        read_flights
    ]
}