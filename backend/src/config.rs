use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::{env};

pub static CONFIG: Lazy<Config> = Lazy::new(Config::load);

pub struct Config {
    pub oauth_github_client_id: Option<String>,
    pub oauth_github_client_secret: Option<String>,
    pub redis_url: Option<String>,
}

fn read_opt_from_env(key: &str) -> Option<String> {
    match env::var(key) {
        Ok(value) => return Some(value),
        Err(_e) => return None,
    }
}

impl Config {
    pub fn load() -> Self {
        Self {
            oauth_github_client_id: read_opt_from_env("OAUTH_GITHUB_CLIENT_ID"),
            oauth_github_client_secret: read_opt_from_env("OAUTH_GITHUB_CLIENT_SECRET"),
            redis_url: read_opt_from_env("REDIS_URL"),
        }
    }
}

pub fn init() {
    dotenv().ok();
    Lazy::force(&CONFIG);
}
