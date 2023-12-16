use axum::routing::post;
use axum::{routing::get, Router};

use crate::day01::exclusive_cube;
use crate::day04::{contest, strength};
use crate::day06::elf_on_a_shelf;
use crate::day07::{bake, cookies};
use crate::day08::weight;
use crate::day_minus_one::make_error;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub fn create_api_router() -> Router {
    let day_minus_one = Router::new().route("/error", get(make_error));
    let day_one = Router::new().route("/*nums", get(exclusive_cube));
    let day_four = Router::new()
        .route("/strength", post(strength))
        .route("/contest", post(contest));
    let day_six = Router::new().route("/", post(elf_on_a_shelf));
    let day_seven = Router::new()
        .route("/decode", get(cookies))
        .route("/bake", get(bake));
    let day_eight = Router::new().route("/weight/:num", get(weight));

    Router::new()
        .route("/", get(hello_world))
        .nest("/-1", day_minus_one)
        .nest("/1", day_one)
        .nest("/4", day_four)
        .nest("/6", day_six)
        .nest("/7", day_seven)
        .nest("/8", day_eight)
}
