use axum::{http::HeaderMap, Json};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct CookieResponse {
    flour: u32,
    #[serde(rename = "chocolate chips")]
    chocolate_chips: u32,
}

pub async fn cookies(headers: HeaderMap) -> Json<CookieResponse> {
    let cookie = headers.get("cookie").unwrap();
    let cookie_value = cookie.to_str().unwrap().split_once("=").unwrap();
    let decoded_raw: Vec<u8> = general_purpose::STANDARD.decode(cookie_value.1).unwrap();
    let decoded = String::from_utf8(decoded_raw).unwrap();

    let res = serde_json::from_str::<CookieResponse>(&decoded).unwrap();

    Json(res)
}
