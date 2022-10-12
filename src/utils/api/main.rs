use std::env;

use lazy_static::lazy_static;

use reqwest::header;

lazy_static! {
    pub static ref API_URL: String = env::var("FILLOABOT_API_URL").unwrap_or_default();
    static ref API_TOKEN: String = env::var("FILLOABOT_API_TOKEN").unwrap_or_default();
}

pub async fn get_client() -> reqwest::Client {
    let mut headers = header::HeaderMap::new();

    let mut auth_value = header::HeaderValue::from_str(format!("Bearer {}", *API_TOKEN).as_str()).expect("Error in header creation");
    auth_value.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth_value);

    headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));
    headers.insert(header::ACCEPT, header::HeaderValue::from_static("application/json"));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build().expect("Error creating client");

    return client
}