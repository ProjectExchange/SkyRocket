table! {
    addresses (id) {
        id -> Integer,
        user_id -> Integer,
        country -> Varchar,
        postal_code -> Integer,
        town -> Varchar,
        street -> Varchar,
        house_number -> Integer,
    }
}

table! {
    bookings (user_id, offer_id) {
        user_id -> Integer,
        offer_id -> Integer,
        seats -> Integer,
    }
}

table! {
    flights (id) {
        id -> Integer,
        offer_id -> Integer,
        departure_icao -> Varchar,
        departure_time -> Datetime,
        arrival_icao -> Varchar,
        arrival_time -> Datetime,
    }
}

table! {
    use diesel::sql_types::{Float, Integer};
    use crate::db::models::CurrencyMapping;
    flights_offers (id) {
        id -> Integer,
        seats -> Integer,
        price -> Float,
        currency -> CurrencyMapping,
    }
}

table! {
    use diesel::sql_types::{BigInt, Float, Integer};
    use crate::db::models::CurrencyMapping;
    flights_offers_with_capacity (id) {
        id -> Integer,
        seats -> Integer,
        occupied -> BigInt,
        price -> Float,
        currency -> CurrencyMapping,
    }
}

table! {
    sessions (id) {
        id -> Integer,
        user_id -> Integer,
        redis_key -> Varchar,
        established -> Datetime,
        data -> Varchar,
    }
}

table! {
    use diesel::sql_types::{Date, Integer, Varchar};
    use crate::db::models::GenderMapping;
    users (id) {
        id -> Integer,
        firstname -> Varchar,
        lastname -> Varchar,
        email -> Varchar,
        birthday -> Date,
        gender -> GenderMapping,
    }
}

table! {
    users_oauth_github (github_id) {
        user_id -> Integer,
        github_id -> Integer,
    }
}

table! {
    use diesel::sql_types::Integer;
    use crate::db::models::RoleMapping;
    users_roles (user_id, role) {
        user_id -> Integer,
        role -> RoleMapping, // Generated Diesel type
    }
}

joinable!(addresses -> users (user_id));
joinable!(bookings -> flights_offers (offer_id));
joinable!(bookings -> users (user_id));
joinable!(flights -> flights_offers (offer_id));
joinable!(sessions -> users (user_id));
joinable!(users_oauth_github -> users (user_id));
joinable!(users_roles -> users (user_id));

allow_tables_to_appear_in_same_query!(
    addresses,
    bookings,
    flights,
    flights_offers,
    sessions,
    users,
    users_oauth_github,
    users_roles,
);
