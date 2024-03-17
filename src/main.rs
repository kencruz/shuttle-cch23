mod day01;
mod day04;
mod day06;
mod day07;
mod day08;
mod day11;
mod day12;
mod day_minus_one;
mod router;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = router::create_api_router();

    Ok(router.into())
}
