use crate::db::models::AuthUser;
use crate::CONFIG;
use async_redis_session::RedisSessionStore;
use async_session::{Session, SessionStore};
use once_cell::sync::Lazy;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::serde::Serialize;

pub static REDIS: Lazy<RedisSessionStore> = Lazy::new(load);

const COOKIE_NAME: &str = "session";
const SESSION_USER_NAME: &str = "user";
const SESSION_GITHUB_ID_NAME: &str = "github_id";

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

/// Retrieve redis session key by the users browser cookies
///
/// # Arguments
///
/// * `cookies` - The cookie jar of the users browser, which holds the redis key
pub fn get_redis_session_key(cookies: &CookieJar<'_>) -> Option<String> {
    cookies
        .get_private(COOKIE_NAME)
        .and_then(|c| c.value().parse().ok())
}

/// Retrieve a session from redis
///
/// # Arguments
///
/// * `redis_key` - The key of the stored redis session
async fn get_redis_session(redis_key: String) -> Option<Session> {
    REDIS.load_session(redis_key).await.unwrap_or_else(|e| {
        eprintln! { "Error retrieving redis session: {}", e };
        None
    })
}

/// Create a session for the current user, store it in redis and save a cookie to the users browser.
/// Returns the redis key to retrieve the session data
///
/// # Arguments
///
/// * `cookies` - The cookie jar of the users browser, which holds the redis key
/// * `user` - The user to save the cookie for. User data is stored within redis
pub async fn set_user(cookies: &CookieJar<'_>, user: AuthUser) -> String {
    let redis_key = add_redis_session(SESSION_USER_NAME, user).await.unwrap();
    add_browser_cookie(cookies, COOKIE_NAME, redis_key.clone());
    redis_key
}

/// Try to find an existing user by its session.
///
/// # Arguments
///
/// * `cookies` - The cookie jar of the users browser, which holds the redis key
/// * `id` - The users GitHub ID
pub async fn set_github_id(cookies: &CookieJar<'_>, id: i32) {
    let redis_key = add_redis_session(SESSION_GITHUB_ID_NAME, id).await;
    add_browser_cookie(cookies, COOKIE_NAME, redis_key.unwrap());
}

/// Try to find an existing user by its session.
///
/// # Arguments
///
/// * `cookies` - The cookie jar of the users browser
pub async fn get_user_from_session(cookies: &CookieJar<'_>) -> Option<AuthUser> {
    let key = get_redis_session_key(cookies)?;
    get_redis_session(key).await?.get(SESSION_USER_NAME)
}

/// Try to find an existing user by its session.
///
/// # Arguments
///
/// * `cookies` - The cookie jar of the users browser
pub async fn get_github_id(cookies: &CookieJar<'_>) -> Option<i32> {
    let key = get_redis_session_key(cookies)?;
    get_redis_session(key).await?.get(SESSION_GITHUB_ID_NAME)
}

/// Destroy the current user session. Session is removed in redis and the users browser
/// cookie is deleted. Returns the `Some(String)` where the content of String
/// is the redis key if successful, `None` otherwise.
///
/// # Arguments
///
/// * `cookies` - The cookie jar of the users browser
pub async fn revoke(cookies: &CookieJar<'_>) -> Option<String> {
    let key = get_redis_session_key(cookies)?;
    cookies.remove_private(Cookie::named(COOKIE_NAME));
    revoke_by_key(key.clone()).await?;
    Some(key)
}

/// Destroy a redis session by the given key.
///
/// # Arguments
///
/// * `redis_key` - The redis key of the stored session
pub async fn revoke_by_key(redis_key: String) -> Option<()> {
    let session = get_redis_session(redis_key).await?;
    REDIS.destroy_session(session).await.ok()
}

pub fn init() {
    Lazy::force(&REDIS);
}
