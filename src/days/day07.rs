use std::collections::HashMap;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BakeRequest {
    recipe: HashMap<String, u32>,
    pantry: HashMap<String, u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BakeResponse {
    cookies: u32,
    pantry: HashMap<String, u32>,
}

fn extract_cookie_value(headers: HeaderMap) -> String {
    let cookie = headers.get("cookie").unwrap();
    let cookie_value = cookie.to_str().unwrap().split_once("=").unwrap();
    let decoded_raw: Vec<u8> = general_purpose::STANDARD.decode(cookie_value.1).unwrap();
    String::from_utf8(decoded_raw).unwrap()
}

pub async fn cookies(headers: HeaderMap) -> Json<CookieResponse> {
    let cookie = extract_cookie_value(headers);

    let res = serde_json::from_str::<CookieResponse>(&cookie).unwrap();

    Json(res)
}

pub async fn bake(headers: HeaderMap) -> Json<BakeResponse> {
    let cookie = extract_cookie_value(headers);
    let bake_request: BakeRequest = serde_json::from_str(&cookie).unwrap();

    let recipe = bake_request.recipe;
    let mut pantry = bake_request.pantry;
    let mut cookie_count = 0;

    let mut break_loop = false;
    while break_loop == false {
        // check if there is enough ingredients for recipe, if not then break loop
        for (k, v) in recipe.clone().drain() {
            let pantry_stock = pantry.get(&k);
            if pantry_stock.is_none() || pantry_stock.unwrap() < &v {
                break_loop = true;
                break;
            }
        }
        if break_loop {
            break;
        }

        // subtract the values from pantry and then add to cookie_count
        for (k, v) in recipe.clone().drain() {
            pantry.entry(k).and_modify(|e| *e -= v);
        }

        cookie_count += 1;
    }

    Json(BakeResponse {
        cookies: cookie_count,
        pantry,
    })
}
