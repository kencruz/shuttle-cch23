use types::AppState;

mod days;
mod router;
mod types;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let shared_state = AppState::new();
    let router = router::create_api_router(shared_state);

    Ok(router.into())
}
