use super::DbResult;
use crate::db::models::role::Role;
use crate::db::models::UserRole;
use crate::db::{schema::users, Db};
use crate::routes::{error, ApiResult};
use crate::session;
use chrono::{NaiveDate, Utc};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
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
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, Deserialize, Serialize, DbEnum, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub enum Gender {
    Male,
    Female,
    Diverse,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, JsonSchema, AsChangeset, Validate)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct NewUser {
    pub firstname: String,
    pub lastname: String,
    #[validate(email)]
    pub email: String,
    #[validate(custom = "is_adult")]
    pub birthday: NaiveDate,
    pub gender: Gender,
}

/// custom validator function to check that a given user is older than 18 years
fn is_adult(birthday: &NaiveDate) -> Result<(), ValidationError> {
    let age_in_days = Utc::now()
        .naive_utc()
        .signed_duration_since(birthday.and_hms(0, 0, 0)).num_days();
    if  age_in_days / 365 > 18 {
        Ok(())
    } else {
        Err(ValidationError::new("User must be older than 18 years"))
    }
}

impl NewUser {
    pub fn is_valid(&self) -> ApiResult<()> {
        self.validate()
            .map_err(|e| error(Status::BadRequest, &e.to_string()))
    }

    pub async fn save(db: &Db, user: NewUser) -> Option<usize> {
        db.run(move |conn| diesel::insert_into(users::table).values(user).execute(conn))
            .await
            .ok()
    }

    pub async fn save_and_return(&self, db: &Db) -> Option<Json<User>> {
        NewUser::save(db, self.clone()).await?;
        User::find_by_email(db, self.email.clone()).await
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
    pub birthday: NaiveDate,
    pub gender: Gender,
}

impl User {
    /// Create a dummy user to satisfy oso policy evaluation
    pub fn dummy(id: i32) -> Self {
        User {
            id,
            firstname: String::new(),
            lastname: String::new(),
            email: String::new(),
            birthday: NaiveDate::from_yo(1970, 1),
            gender: Gender::Male,
        }
    }

    /// Can be called after a user was created. Indicates whether the inserted user was the first
    /// one created and should have special privileges.
    pub async fn is_first(&self, db: &Db) -> bool {
        let res_count = db
            .run(move |conn| users::table.count().get_result(conn))
            .await;
        Ok(1) == res_count
    }

    pub async fn attach_role(&self, db: &Db, role: Role) -> DbResult {
        UserRole::add(db, self.clone(), role).await
    }

    pub async fn get_all(db: &Db) -> Option<Json<Vec<User>>> {
        db.run(move |conn| users::table.load::<User>(conn))
            .await
            .map(Json)
            .ok()
    }

    pub async fn update_and_return(db: &Db, id: i32, new_user: NewUser) -> Option<Json<User>> {
        db.run(move |conn| {
            diesel::update(users::table.filter(users::id.eq(id)))
                .set(new_user)
                .execute(conn)
        })
        .await
        .ok()?;

        User::find_by_id(db, id).await
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
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, PolarClass)]
#[serde(crate = "rocket::serde")]
pub struct AuthUser {
    #[polar(attribute)]
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub birthday: NaiveDate,
    pub gender: Gender,
    #[polar(attribute)]
    pub roles: Vec<Role>,
}

impl AuthUser {
    pub async fn by_user_id(db: &Db, id: i32) -> Option<Self> {
        let user = User::find_by_id(db, id).await?;
        let roles = UserRole::all_from_user(db, user.clone()).await;

        Some(AuthUser::new(user, roles))
    }

    pub async fn by_email(db: &Db, email: String) -> Option<Self> {
        let user = User::find_by_email(db, email).await?;
        let roles = UserRole::all_from_user(db, user.clone()).await;

        Some(AuthUser::new(user, roles))
    }

    pub fn new(user: Json<User>, roles: Vec<Role>) -> Self {
        AuthUser {
            id: user.id,
            firstname: user.firstname.clone(),
            lastname: user.lastname.clone(),
            email: user.email.clone(),
            birthday: user.birthday,
            gender: user.gender.clone(),
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
