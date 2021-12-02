use crate::models::User;
use crate::CONFIG;
use async_redis_session::RedisSessionStore;
use async_session::{Session, SessionStore};
use once_cell::sync::Lazy;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::serde::Serialize;

pub static REDIS: Lazy<RedisSessionStore> = Lazy::new(load);

const COOKIE_NAME: &'static str = "session";
const SESSION_USER_NAME: &'static str = "user";
const SESSION_GITHUB_ID_NAME: &'static str = "github_id";

/// initialize redis with connection string from config
fn load() -> RedisSessionStore {
    let url = CONFIG
        .redis_url
        .as_ref()
        .expect("Failed to connect to Redis");
    RedisSessionStore::new(url.to_string()).unwrap()
}

/// safe a secure cookie to the users browsers
///
/// # Arguments
///
/// * `cookies` - The cookie jar of the users browser
/// * `key` - The name of the cookie
/// * `value` - The value of the cookie. It is also the key used to retrieve a session from redis
fn add_browser_cookie(cookies: &CookieJar<'_>, key: &'static str, value: String) {
    // ToDo: Expire?
    let cookie = Cookie::build(key, value)
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict);
    cookies.add_private(cookie.finish())
}

/// creates a new session and stores it in redis
async fn add_redis_session(key: &str, value: impl Serialize) -> Option<String> {
    let mut session = Session::new();
    session.insert(key, value).unwrap();
    REDIS.store_session(session).await.unwrap()
}

/// retrieve a session stored in redis
///
/// # Arguments
///
/// * `cookies` - The cookie jar of the users browser, which holds the redis key
async fn get_redis_session(cookies: &CookieJar<'_>) -> Option<Session> {
    let cookie_value = cookies
        .get_private(COOKIE_NAME)
        .and_then(|c| c.value().parse().ok())?;
    REDIS.load_session(cookie_value).await.unwrap_or_else(|e| {
        eprintln! { "Error retrieving redis session: {}", e };
        None
    })
}

/// create a session for the current user, store it in redis and save a cookie to the users browser
///
/// # Arguments
///
/// * `cookies` - The cookie jar of the users browser, which holds the redis key
/// * `user` - The user to save the cookie for. User data is stored within redis
pub async fn set_user(cookies: &CookieJar<'_>, user: User) {
    let cookie_option = add_redis_session(SESSION_USER_NAME, user).await;
    add_browser_cookie(cookies, COOKIE_NAME, cookie_option.unwrap());
}

/// Try to find an existing user by its session.
///
/// # Arguments
///
/// * `cookies` - The cookie jar of the users browser, which holds the redis key
/// * `id` - The users GitHub ID
pub async fn set_github_id(cookies: &CookieJar<'_>, id: i32) {
    let cookie_option = add_redis_session(SESSION_GITHUB_ID_NAME, id).await;
    add_browser_cookie(cookies, COOKIE_NAME, cookie_option.unwrap());
}

/// Try to find an existing user by its session.
///
/// # Arguments
///
/// * `cookies` - The cookie jar of the users browser
pub async fn get_user_from_session(cookies: &CookieJar<'_>) -> Option<User> {
    get_redis_session(cookies).await?.get(SESSION_USER_NAME)
}

/// Try to find an existing user by its session.
///
/// # Arguments
///
/// * `cookies` - The cookie jar of the users browser
pub async fn get_github_id(cookies: &CookieJar<'_>) -> Option<i32> {
    get_redis_session(cookies)
        .await?
        .get(SESSION_GITHUB_ID_NAME)
}

/// Destroy the current user session. Session is removed in redis and the users browser
/// cookie is deleted. Returns `Some(())` if successful, `None` otherwise.
///
/// # Arguments
///
/// * `cookies` - The cookie jar of the users browser
pub async fn revoke(cookies: &CookieJar<'_>) -> Option<()> {
    let session = get_redis_session(cookies).await?;
    REDIS.destroy_session(session).await.ok()?;
    cookies.remove_private(Cookie::named(COOKIE_NAME));
    Some(())
}

pub fn init() {
    Lazy::force(&REDIS);
}
