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
        .route("/api/building/:id", get(handler::building::get))
        .route("/api/building/:id", patch(handler::building::patch))
        .route("/api/building/:id", delete(handler::building::delete))
        .route("/api/fortress", get(handler::fortress::get_all))
        .route("/api/fortress", post(handler::fortress::post))
        .route("/api/fortress/:id", get(handler::fortress::get))
        .route("/api/fortress/:id", patch(handler::fortress::patch))
        .route("/api/fortress/:id", delete(handler::fortress::delete))
        .route(
            "/api/fortress/:fortress_id/building",
            get(handler::building::get_by_fortress),
        )
        .with_state(pool);
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let tcp_listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(tcp_listener, app).await.unwrap();
}
