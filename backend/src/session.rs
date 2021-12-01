use crate::models::User;
use rocket::http::{CookieJar,SameSite,Cookie};
use async_redis_session::RedisSessionStore;
use async_session::{Session, SessionStore};
use once_cell::sync::Lazy;
use crate::CONFIG;

pub static REDIS: Lazy<RedisSessionStore> = Lazy::new(load);

fn load() -> RedisSessionStore {
    let url = CONFIG.redis_url.as_ref().expect("Failed to connect to Redis");
    RedisSessionStore::new(url.to_string()).unwrap()
}

pub async fn create_session(cookies: &CookieJar<'_>, user: User) {
    let mut session = Session::new();
    session.insert("user", user).unwrap();
    let cookie_value = REDIS.store_session(session).await.unwrap();
    // ToDo: Expire?
    let cookie = Cookie::build("session", cookie_value.unwrap())
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict);
    cookies.add_private(cookie.finish())
}

pub async fn get_user_from_session(cookies: &CookieJar<'_>) -> Option<User> {
    let cookie_value = cookies.get_private("session")?.to_string();
    let session = REDIS.load_session(cookie_value).await.unwrap()?;
    session.get("user")
}

pub fn init() {
    Lazy::force(&REDIS);
}
