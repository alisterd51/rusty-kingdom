pub mod building;
pub mod fortress;

use axum::http::StatusCode;
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};

type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
