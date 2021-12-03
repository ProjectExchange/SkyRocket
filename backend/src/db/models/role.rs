use crate::db::models::User;
use crate::db::schema::users_roles;
use crate::db::Db;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Associations, Deserialize, Serialize, Identifiable, Queryable, Insertable,
)]
#[belongs_to(User, foreign_key = "user_id")]
#[serde(crate = "rocket::serde")]
#[table_name = "users_roles"]
#[primary_key(user_id, role_id)]
pub struct UserRole {
    pub user_id: i32,
    pub role_id: i32,
}

impl UserRole {
    pub async fn all_from_user(db: &Db, user: User) -> Vec<i32> {
        db.run(move |conn| {
            UserRole::belonging_to(&user)
                .select(users_roles::role_id)
                .load::<i32>(conn)
        })
        .await
        .unwrap_or(Vec::new())
    }
}
