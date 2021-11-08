use super::schema::users;

use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
}
