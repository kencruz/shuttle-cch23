use axum::{extract, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Reindeer {
    name: String,
    strength: u32,
    speed: Option<f32>,
    height: Option<u32>,
    antler_width: Option<u32>,
    snow_magic_power: Option<u32>,
    favorite_food: Option<String>,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: Option<u32>,
}

impl Clone for Reindeer {
    fn clone(&self) -> Reindeer {
        Reindeer {
            name: self.name.clone(),
            strength: self.strength,
            speed: self.speed,
            height: self.height,
            antler_width: self.antler_width,
            snow_magic_power: self.snow_magic_power,
            favorite_food: self.favorite_food.clone(),
            candies_eaten_yesterday: self.candies_eaten_yesterday,
        }
    }
}

#[derive(Serialize)]
pub struct ContestResponse {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

pub async fn strength(extract::Json(payload): extract::Json<Vec<Reindeer>>) -> String {
    let sum = payload.into_iter().fold(0, |sum, r| r.strength + sum);
    format!("{}", sum)
}

pub async fn contest(
    extract::Json(payload): extract::Json<Vec<Reindeer>>,
) -> Json<ContestResponse> {
    let mut fastest: Option<Reindeer> = None;
    let mut tallest: Option<Reindeer> = None;
    let mut magician: Option<Reindeer> = None;
    let mut consumer: Option<Reindeer> = None;

    payload.into_iter().for_each(|r| {
        if fastest.is_none() || &fastest.as_ref().unwrap().speed.unwrap() < &r.speed.unwrap() {
            fastest = Some(r.clone());
        }

        if tallest.is_none() || &tallest.as_ref().unwrap().height.unwrap() < &r.height.unwrap() {
            tallest = Some(r.clone());
        }

        if magician.is_none()
            || &magician.as_ref().unwrap().snow_magic_power.unwrap() < &r.snow_magic_power.unwrap()
        {
            magician = Some(r.clone());
        }

        if consumer.is_none()
            || &consumer.as_ref().unwrap().candies_eaten_yesterday.unwrap()
                < &r.candies_eaten_yesterday.unwrap()
        {
            consumer = Some(r.clone());
        }
    });

    Json(ContestResponse {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.clone().unwrap().strength,
            fastest.unwrap().name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.clone().unwrap().name,
            tallest.unwrap().antler_width.unwrap(),
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.clone().unwrap().name,
            magician.unwrap().snow_magic_power.unwrap(),
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.clone().unwrap().name,
            consumer.unwrap().favorite_food.unwrap()
        ),
    })
}
