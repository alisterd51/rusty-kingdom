pub mod building;
pub mod fortress;

use axum::{http::StatusCode, response::IntoResponse};

#[derive(serde::Serialize, serde::Deserialize)]
enum Status {
    Up,
    Down,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct AppStatus {
    pub status: Status,
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub async fn health() -> impl IntoResponse {
    axum::Json(AppStatus { status: Status::Up })
}
