use super::schema::users;

use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
}
