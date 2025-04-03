mod handler;

use axum::{Router, routing::get};
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

#[derive(Clone)]
struct AppState {
    redis_client: redis::Client,
    diesel_pool: Pool,
}

#[tokio::main]
async fn main() {
    let redis_url = std::env::var("REDIS_URL").unwrap();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    let app_state = AppState {
        redis_client: redis::Client::open(redis_url).unwrap(),
        diesel_pool: bb8::Pool::builder().build(manager).await.unwrap(),
    };
    let app = Router::new()
        .route(
            "/api/building",
            get(handler::building::get_all).post(handler::building::post),
        )
        .route(
            "/api/building/{building_id}",
            get(handler::building::get)
                .patch(handler::building::patch)
                .delete(handler::building::delete),
        )
        .route(
            "/api/fortress",
            get(handler::fortress::get_all).post(handler::fortress::post),
        )
        .route(
            "/api/fortress/{fortress_id}",
            get(handler::fortress::get)
                .patch(handler::fortress::patch)
                .delete(handler::fortress::delete),
        )
        .route(
            "/api/fortress/{fortress_id}/building",
            get(handler::building::get_by_fortress).delete(handler::building::delete_by_fortress),
        )
        .with_state(app_state);
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    let tcp_listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(tcp_listener, app).await.unwrap();
}
