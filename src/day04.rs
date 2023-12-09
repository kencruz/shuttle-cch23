use axum::{extract, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct StrengthReindeer {
    name: String,
    strength: u32,
}

#[derive(Deserialize)]
pub struct ContestReindeer {
    name: String,
    strength: u32,
    speed: f32,
    height: u32,
    antler_width: u32,
    snow_magic_power: u32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: u32,
}

#[derive(Serialize)]
pub struct ContestResponse {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

pub async fn strength(extract::Json(payload): extract::Json<Vec<StrengthReindeer>>) -> String {
    let sum = payload.into_iter().fold(0, |sum, r| r.strength + sum);
    format!("{}", sum)
}

pub async fn contest(
    extract::Json(payload): extract::Json<Vec<ContestReindeer>>,
) -> Json<ContestResponse> {
    let fastest = &payload
        .iter()
        .max_by(|a, b| a.speed.partial_cmp(&b.speed).unwrap())
        .unwrap();

    let tallest = &payload
        .iter()
        .max_by(|a, b| a.height.partial_cmp(&b.height).unwrap())
        .unwrap();

    let magician = &payload
        .iter()
        .max_by(|a, b| a.snow_magic_power.partial_cmp(&b.snow_magic_power).unwrap())
        .unwrap();

    let consumer = &payload
        .iter()
        .max_by(|a, b| {
            a.candies_eaten_yesterday
                .partial_cmp(&b.candies_eaten_yesterday)
                .unwrap()
        })
        .unwrap();

    Json(ContestResponse {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.strength, fastest.name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name, tallest.antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.name, magician.snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.name, consumer.favorite_food
        ),
    })
}
