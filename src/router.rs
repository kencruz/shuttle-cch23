use axum::routing::post;
use axum::{routing::get, Router};
use tower_http::services::ServeDir;

use crate::days::day01::exclusive_cube;
use crate::days::day04::{contest, strength};
use crate::days::day06::elf_on_a_shelf;
use crate::days::day07::{bake, cookies};
use crate::days::day08::{weight, drop};
use crate::days::day11::red_pixels;
use crate::days::day12::{load_packet, store_packet, ulids_to_uuids};
use crate::days::day_minus_one::make_error;
use crate::types::AppState;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub fn create_api_router(shared_state: AppState) -> Router {

    let day_minus_one = Router::new().route("/error", get(make_error));
    let day_one = Router::new().route("/*nums", get(exclusive_cube));
    let day_four = Router::new()
        .route("/strength", post(strength))
        .route("/contest", post(contest));
    let day_six = Router::new().route("/", post(elf_on_a_shelf));
    let day_seven = Router::new()
        .route("/decode", get(cookies))
        .route("/bake", get(bake));
    let day_eight = Router::new()
        .route("/weight/:num", get(weight))
        .route("/drop/:num", get(drop));
    let day_eleven = Router::new()
        .route("/red_pixels", post(red_pixels))
        .nest_service("/assets", ServeDir::new("assets"));
    let day_twelve = Router::new()
        .route("/save/:packet", post(store_packet))
        .route("/load/:packet", get(load_packet))
        .route("/ulids", post(ulids_to_uuids))
        .with_state(shared_state);

    Router::new()
        .route("/", get(hello_world))
        .nest("/-1", day_minus_one)
        .nest("/1", day_one)
        .nest("/4", day_four)
        .nest("/6", day_six)
        .nest("/7", day_seven)
        .nest("/8", day_eight)
        .nest("/11", day_eleven)
        .nest("/12", day_twelve)
}
