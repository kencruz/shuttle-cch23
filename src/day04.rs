use axum::{extract, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Reindeer {
    name: String,
    strength: u32,
}

pub async fn strength(extract::Json(payload): extract::Json<Vec<Reindeer>>) -> String {
    let sum = payload.into_iter().fold(0, |sum, r| r.strength + sum);
    format!("{}", sum)
}
