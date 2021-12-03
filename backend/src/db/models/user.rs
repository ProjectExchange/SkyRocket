use crate::db::models::role::Role;
use crate::db::models::UserRole;
use crate::db::{schema::users, Db};
use crate::session;
use diesel::prelude::*;
use oso::{Oso, PolarClass};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::Request;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct NewUser {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

impl NewUser {
    pub async fn save(db: &Db, user: NewUser) -> Option<usize> {
        db.run(move |conn| diesel::insert_into(users::table).values(user).execute(conn))
            .await
            .ok()
    }
}

#[derive(Debug, Clone, PolarClass, Deserialize, Serialize, Identifiable, Queryable, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct User {
    #[polar(attribute)]
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

impl User {
    pub async fn get_all(db: &Db) -> Option<Json<Vec<User>>> {
        db.run(move |conn| users::table.load::<User>(conn))
            .await
            .map(Json)
            .ok()
    }

    pub async fn delete(db: &Db, id: i32) -> Option<()> {
        db.run(move |conn| {
            diesel::delete(users::table)
                .filter(users::id.eq(id))
                .execute(conn)
        })
        .await
        .ok()
        .map_or_else(|| Some(()), |_e| None)
    }

    pub async fn find_by_id(db: &Db, id: i32) -> Option<Json<Self>> {
        db.run(move |conn| users::table.find(id).first(conn))
            .await
            .map(Json)
            .ok()
    }

    pub async fn find_by_email(db: &Db, email: String) -> Option<Json<Self>> {
        db.run(move |conn| users::table.filter(users::email.eq(email)).first(conn))
            .await
            .map(Json)
            .ok()
    }

    pub async fn save_and_return(db: &Db, user: NewUser) -> Option<Json<Self>> {
        NewUser::save(db, user.clone()).await?;
        User::find_by_email(db, user.email).await
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, PolarClass)]
#[serde(crate = "rocket::serde")]
pub struct AuthUser {
    #[polar(attribute)]
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    #[polar(attribute)]
    pub roles: Vec<Role>,
}

impl AuthUser {
    pub async fn by_user_id(db: &Db, id: i32) -> Option<Self> {
        let user = User::find_by_id(db, id).await?;
        let roles = UserRole::all_from_user(db, user.clone()).await;

        Some(AuthUser::new(user, roles))
    }

    pub fn new(user: Json<User>, roles: Vec<Role>) -> Self {
        AuthUser {
            id: user.id,
            firstname: user.firstname.clone(),
            lastname: user.lastname.clone(),
            email: user.email.clone(),
            roles,
        }
    }
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for AuthUser {
    type Error = String;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(user) = session::get_user_from_session(request.cookies()).await {
            Outcome::Success(user)
        } else {
            Outcome::Failure((Status::Forbidden, "Not logged in".to_owned()))
        }
    }
}

impl<'r> OpenApiFromRequest<'r> for AuthUser {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}

pub(super) fn register_polar_classes(oso: &mut Oso) -> oso::Result<()> {
    oso.register_class(User::get_polar_class())?;
    oso.register_class(AuthUser::get_polar_class())
}
