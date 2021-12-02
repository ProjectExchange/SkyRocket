use crate::db::{schema::users_oauth_github, Db};
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[table_name = "users_oauth_github"]
pub struct GitHubOAuthUser {
    pub user_id: i32,
    pub github_id: i32,
}

impl GitHubOAuthUser {
    pub async fn find_by_id(db: &Db, id: i32) -> Option<Json<Self>> {
        db.run(move |conn| users_oauth_github::table.find(id).first(conn))
            .await
            .map(Json)
            .ok()
    }

    pub async fn save(db: &Db, user: GitHubOAuthUser) -> Option<usize> {
        db.run(move |conn| {
            diesel::insert_into(users_oauth_github::table)
                .values(user)
                .execute(conn)
        })
        .await
        .ok()
    }
}
