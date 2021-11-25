use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::{env, process::exit};

pub static CONFIG: Lazy<Config> = Lazy::new(Config::load);

pub struct Config {
    pub oauth_github_client_id: String,
    pub oauth_github_client_secret: String,
}

fn read_from_env(key: &str) -> String {
    match env::var(key) {
        Ok(value) => return value,
        Err(_e) => {
            println!("Error loading config:\n\t{} {:?}\n", key, _e);
            exit(1);
        }
    }
}

impl Config {
    pub fn load() -> Self {
        Self {
            oauth_github_client_id: read_from_env("OAUTH_GITHUB_CLIENT_ID"),
            oauth_github_client_secret: read_from_env("OAUTH_GITHUB_CLIENT_SECRET"),
        }

    }
}

pub fn init() {
    dotenv().ok();
    Lazy::force(&CONFIG);
}
