use axum::extract::Path;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
struct SolveForTimeError;

impl fmt::Display for SolveForTimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No real solutions")
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonStats {
    pub weight: i64,
}

fn solve_for_time(
    distance: f64,
    initial_velocity: f64,
    acceleration: f64,
) -> Result<f64, SolveForTimeError> {
    // Calculate the discriminant
    let discriminant = initial_velocity * initial_velocity + 2.0 * acceleration * distance;

    // Check if the discriminant is negative
    if discriminant < 0.0 {
        // No real solutions
        return Err(SolveForTimeError);
    }

    // Calculate both possible solutions for t
    let t1 = (-initial_velocity + f64::sqrt(discriminant)) / acceleration;
    let t2 = (-initial_velocity - f64::sqrt(discriminant)) / acceleration;

    if t1 < 0.0 && t2 < 0.0 {
        return Err(SolveForTimeError);
    }

    Ok(f64::max(t1, t2))
}

pub async fn weight(Path(params): Path<HashMap<String, String>>) -> String {
    println!("{:?}", params);
    let pokedex_num = params.get("num").unwrap().parse::<u32>().unwrap();
    let api_res = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{}", pokedex_num))
        .await
        .unwrap();

    let body = api_res.text().await.unwrap();
    let data: PokemonStats = serde_json::from_str(&body).unwrap();

    (data.weight / 10).to_string()
}

pub async fn drop(Path(params): Path<HashMap<String, String>>) -> String {
    let pokedex_num = params.get("num").unwrap().parse::<u32>().unwrap();
    let api_res = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{}", pokedex_num))
        .await
        .unwrap();

    let body = api_res.text().await.unwrap();
    let data: PokemonStats = serde_json::from_str(&body).unwrap();

    let mass = (data.weight / 10) as f64;
    let acceleration = 9.825;
    let time = solve_for_time(10.0, 0.0, acceleration).unwrap();
    let velocity = 0.0 + acceleration * time;

    format!("{}", mass * velocity)
}
