use axum::{extract::Path, http::StatusCode, routing::get, Router};
use std::collections::HashMap;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn make_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn exclusive_cube(Path(params): Path<HashMap<String, String>>) -> String {
    let mut nums = params
        .get("nums")
        .unwrap()
        .split("/")
        .map(|n| n.parse::<u32>().unwrap())
        .into_iter();

    let first = nums.next().unwrap();
    let xor = nums.fold(first, |n, acc| n ^ acc);
    let cubed = xor.pow(3);

    format!("{:?}\n", cubed)
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let day_minus_one = Router::new().route("/error", get(make_error));
    let day_one = Router::new().route("/*nums", get(exclusive_cube));

    let router = Router::new()
        .route("/", get(hello_world))
        .nest("/-1", day_minus_one)
        .nest("/1", day_one);

    Ok(router.into())
}
