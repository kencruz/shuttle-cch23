use axum::{extract::Path, routing::get, Router};

use crate::day01::exclusive_cube;
use crate::day_minus_one::make_error;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub fn create_api_router() -> Router {
    let day_minus_one = Router::new().route("/error", get(make_error));
    let day_one = Router::new().route("/*nums", get(exclusive_cube));

    Router::new()
        .route("/", get(hello_world))
        .nest("/-1", day_minus_one)
        .nest("/1", day_one)
}
