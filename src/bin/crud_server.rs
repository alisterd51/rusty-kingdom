use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use rusty_kingdom::handler;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL").unwrap();

    let manager =
        deadpool_diesel::postgres::Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .unwrap();

    let app = Router::new()
        .route("/api/building", get(handler::building::get_all))
        .route("/api/building", post(handler::building::post))
        .route("/api/building/:building_id", get(handler::building::get))
        .route(
            "/api/building/:building_id",
            patch(handler::building::patch),
        )
        .route(
            "/api/building/:building_id",
            delete(handler::building::delete),
        )
        .route("/api/fortress", get(handler::fortress::get_all))
        .route("/api/fortress", post(handler::fortress::post))
        .route("/api/fortress/:fortress_id", get(handler::fortress::get))
        .route(
            "/api/fortress/:fortress_id",
            patch(handler::fortress::patch),
        )
        .route(
            "/api/fortress/:fortress_id",
            delete(handler::fortress::delete),
        )
        .route(
            "/api/fortress/:fortress_id/building",
            get(handler::building::get_by_fortress),
        )
        .route(
            "/api/fortress/:fortress_id/building",
            delete(handler::building::delete_by_fortress),
        )
        .with_state(pool);
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let tcp_listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(tcp_listener, app).await.unwrap();
}
