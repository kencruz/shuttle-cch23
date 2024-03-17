use axum::http::StatusCode;

pub async fn make_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}
