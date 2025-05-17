use crate::handler;
use axum::{Router, routing::get};
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};
#[cfg(feature = "tracing")]
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
#[cfg(feature = "tracing")]
use tracing::Level;

type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

fn fortress() -> Router<Pool> {
    Router::new()
        .route(
            "/",
            get(handler::fortress::get_all).post(handler::fortress::post),
        )
        .route(
            "/{fortress_id}",
            get(handler::fortress::get)
                .patch(handler::fortress::patch)
                .delete(handler::fortress::delete),
        )
        .route(
            "/{fortress_id}/building",
            get(handler::building::get_by_fortress).delete(handler::building::delete_by_fortress),
        )
}
fn building() -> Router<Pool> {
    Router::new()
        .route(
            "/",
            get(handler::building::get_all).post(handler::building::post),
        )
        .route(
            "/{building_id}",
            get(handler::building::get)
                .patch(handler::building::patch)
                .delete(handler::building::delete),
        )
}

fn api() -> Router<Pool> {
    Router::new()
        .nest("/fortress", fortress())
        .nest("/building", building())
}

#[cfg(feature = "tracing")]
pub async fn router() -> Router {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let manager =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = bb8::Pool::builder().build(manager).await.unwrap();
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    Router::new()
        .nest("/api", api())
        .layer(trace_layer)
        .with_state(pool)
        .route("/health", get(handler::health))
}

#[cfg(not(feature = "tracing"))]
pub async fn router() -> Router {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let manager =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = bb8::Pool::builder().build(manager).await.unwrap();

    Router::new()
        .nest("/api", api())
        .with_state(pool)
        .route("/health", get(handler::health))
}
