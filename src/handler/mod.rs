pub mod building;
pub mod fortress;
pub mod game;

use axum::http::StatusCode;

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
