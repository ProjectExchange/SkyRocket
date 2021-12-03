use crate::db::models::User;
use crate::db::schema::users_roles;
use crate::db::Db;
use std::fmt;
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use oso::{PolarValue, ToPolar};
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Debug, Clone, Deserialize, Serialize, DbEnum, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub enum Role {
    Admin,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ToPolar for Role {
    fn to_polar(self) -> PolarValue {
        PolarValue::String(self.to_string())
    }
}

#[derive(Debug, Clone, Associations, Deserialize, Serialize, Queryable, Insertable)]
#[belongs_to(User, foreign_key = "user_id")]
#[serde(crate = "rocket::serde")]
#[table_name = "users_roles"]
pub struct UserRole {
    pub user_id: i32,
    pub role: Role,
}

impl UserRole {
    pub async fn all_from_user(db: &Db, user: User) -> Vec<Role> {
        db.run(move |conn| {
            users_roles::table
                .filter(users_roles::user_id.eq(user.id))
                .select(users_roles::role)
                .load::<Role>(conn)
        })
        .await
        .unwrap_or(Vec::new())
    }
}
