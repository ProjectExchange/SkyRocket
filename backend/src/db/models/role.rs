use crate::db::models::User;
use crate::db::schema::users_roles;
use crate::db::Db;
use crate::session;
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use oso::{PolarValue, ToPolar};
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::serde::{Deserialize, Serialize};
use rocket::Request;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::request::OpenApiFromRequest;
use rocket_okapi::request::RequestHeaderInput;
use std::fmt;

#[derive(Debug, Clone, Deserialize, Serialize, DbEnum, PartialEq, JsonSchema)]
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
    pub fn new(user: User, role: Role) -> Self {
        UserRole {
            user_id: user.id,
            role,
        }
    }

    pub async fn add(db: &Db, user: User, role: Role) -> super::DbResult {
        db.run(move |conn| {
            diesel::insert_into(users_roles::table)
                .values(UserRole::new(user, role))
                .execute(conn)
        })
        .await
    }

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

/// simple helper function to check if the current user (if any) has a specified role
async fn session_has_role(request: &Request<'_>, role: Role) -> bool {
    if let Some(user) = session::get_user_from_session(request.cookies()).await {
        user.roles.contains(&role)
    } else {
        false
    }
}

/// Dummy role, used for RBAC of Rocket API endpoints
pub struct AdminRole {}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for AdminRole {
    type Error = String;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        if session_has_role(request, Role::Admin).await {
            Outcome::Success(AdminRole {})
        } else {
            Outcome::Forward(())
        }
    }
}

impl<'r> OpenApiFromRequest<'r> for AdminRole {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}
