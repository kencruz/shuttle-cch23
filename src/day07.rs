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

#[derive(Debug, Serialize, Deserialize)]
pub struct BakeRequest {
    recipe: BakeRecipe,
    pantry: BakeRecipe,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BakeRecipe {
    flour: u32,
    sugar: u32,
    butter: u32,
    #[serde(rename = "baking powder")]
    baking_powder: u32,
    #[serde(rename = "chocolate chips")]
    chocolate_chips: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BakeResponse {
    cookies: u32,
    pantry: BakeRecipe,
}

impl BakeRecipe {
    pub fn as_array(&self) -> [u32; 5] {
        [
            self.flour.clone(),
            self.sugar.clone(),
            self.butter.clone(),
            self.baking_powder.clone(),
            self.chocolate_chips.clone(),
        ]
    }
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
    let bake_request: BakeRequest = serde_json::from_str::<BakeRequest>(&cookie).unwrap();

    let (pantry, recipe) = (bake_request.pantry, bake_request.recipe);
    let pantry_recipe_pair = std::iter::zip(pantry.as_array(), recipe.as_array());

    let cookie_count = pantry_recipe_pair.clone().fold(None, |acc, el| {
        if acc.is_none() {
            return Some(el.0 / el.1);
        }

        Some(acc.unwrap().min(el.0 / el.1))
    });

    let remaining: [u32; 5] = pantry_recipe_pair
        .map(|(p, r)| p - r * cookie_count.unwrap())
        .collect::<Vec<u32>>()
        .as_slice()
        .try_into()
        .unwrap();

    Json(BakeResponse {
        cookies: cookie_count.unwrap(),
        pantry: BakeRecipe {
            flour: remaining[0],
            sugar: remaining[1],
            butter: remaining[2],
            baking_powder: remaining[3],
            chocolate_chips: remaining[4],
        },
    })
}
