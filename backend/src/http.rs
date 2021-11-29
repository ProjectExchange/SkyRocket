use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, USER_AGENT};
use rocket::serde::{DeserializeOwned, Serialize};

pub async fn post<T: DeserializeOwned, U: Serialize + ?Sized>(
    url: &str,
    data: &U,
) -> Result<T, reqwest::Error> {
    Ok(reqwest::Client::new()
        .post(url)
        .header("accept", "application/json")
        .json(data)
        .send()
        .await?
        .json::<T>()
        .await?)
}

pub async fn get<T: DeserializeOwned>(url: &str, access_token: &str) -> Result<T, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/vnd.github.v3+json".parse().unwrap());
    headers.insert(USER_AGENT, "Mozilla/5.0".parse().unwrap());
    headers.insert(
        AUTHORIZATION,
        (format! { "token {}", access_token }).parse().unwrap(),
    );

    Ok(reqwest::Client::new()
        .get(url)
        .headers(headers)
        .send()
        .await?
        .json::<T>()
        .await?)
}
