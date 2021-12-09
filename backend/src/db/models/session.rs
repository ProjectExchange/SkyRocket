use super::DbResult;
use crate::db::models::{AuthUser, User};
use crate::db::{schema::sessions, Db};
use crate::routes::{error, ApiResult, UserAgent};
use crate::session as browser_session;
use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::prelude::*;
use oso::PolarClass;
use rocket::http::CookieJar;
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(
    Associations,
    Clone,
    Debug,
    Deserialize,
    PolarClass,
    Identifiable,
    JsonSchema,
    Queryable,
    Serialize,
)]
#[serde(crate = "rocket::serde")]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "sessions"]
pub struct Session {
    pub id: i32,
    #[polar(attribute)]
    pub user_id: i32,
    pub established: NaiveDateTime,
    pub data: String,
}

impl Session {
    /// Create a dummy session for a user with the given id. This function is
    /// used to construct an address object for use with oso policies.
    pub fn dummy_for_user(user_id: i32) -> Self {
        Session {
            id: 0,
            user_id,
            established: NaiveDateTime::from_timestamp(0, 0),
            data: String::new(),
        }
    }

    pub async fn all_from_user(db: &Db, user_id: i32) -> Vec<Self> {
        db.run(move |conn| {
            Session::belonging_to(&User::dummy(user_id))
                .select((
                    sessions::id,
                    sessions::user_id,
                    sessions::established,
                    sessions::data,
                ))
                .load(conn)
        })
        .await
        .unwrap_or_else(|_| Vec::new())
    }

    pub async fn get_redis_key_by_id(db: &Db, session_id: i32) -> Option<String> {
        db.run(move |conn| {
            sessions::table
                .select(sessions::redis_key)
                .find(session_id)
                .first(conn)
        })
        .await
        .ok()
    }

    async fn delete_by_redis_key(db: &Db, redis_key: String) -> ApiResult<()> {
        db.run(move |conn| {
            diesel::delete(sessions::table)
                .filter(sessions::redis_key.eq(redis_key))
                .execute(conn)
        })
        .await
        .map_or_else(
            |e| Err(error(e, Status::NotFound, "Session not found")),
            |_| Ok(()),
        )
    }

    pub async fn delete_by_id(db: &Db, session_id: i32) -> ApiResult<()> {
        let redis_key = Session::get_redis_key_by_id(db, session_id)
            .await
            .ok_or_else(|| error("", Status::NotFound, "Session not found"))?;

        browser_session::revoke_by_key(redis_key.clone())
            .await
            .ok_or_else(|| error("", Status::NotFound, "Session not found"))?;

        Session::delete_by_redis_key(db, redis_key).await
    }

    pub async fn delete_by_cookie(db: &Db, cookies: &CookieJar<'_>) -> ApiResult<()> {
        let key = browser_session::revoke(cookies)
            .await
            .ok_or_else(|| error("", Status::Forbidden, "No session to revoke"))?;

        Session::delete_by_redis_key(db, key).await
    }

    pub async fn save<'a>(
        db: &Db,
        cookies: &CookieJar<'_>,
        ua: UserAgent,
        user: AuthUser,
    ) -> DbResult {
        let redis_key = browser_session::set_user(cookies, user.clone()).await;
        let new_session = NewSession {
            redis_key,
            established: Utc::now().naive_utc(),
            data: ua.0,
        };
        db.run(move |conn| {
            diesel::insert_into(sessions::table)
                .values(&InsertableSession::new(new_session, user.id))
                .execute(conn)
        })
        .await
    }
}

pub struct NewSession {
    redis_key: String,
    established: NaiveDateTime,
    data: String,
}

#[derive(AsChangeset, Clone, Debug, Deserialize, Insertable, Serialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "sessions"]
struct InsertableSession {
    user_id: i32,
    redis_key: String,
    established: NaiveDateTime,
    data: String,
}

impl InsertableSession {
    pub fn new(new_session: NewSession, user_id: i32) -> InsertableSession {
        InsertableSession {
            user_id,
            redis_key: new_session.redis_key,
            established: new_session.established,
            data: new_session.data,
        }
    }
}

pub(super) fn register_polar_classes(oso: &mut oso::Oso) -> oso::Result<()> {
    oso.register_class(Session::get_polar_class())
}
