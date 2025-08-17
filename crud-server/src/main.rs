mod handler;

use axum::{Router, routing::get};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let manager =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = bb8::Pool::builder().build(manager).await.unwrap();
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
        .with_state(pool);
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    let tcp_listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(tcp_listener, app).await.unwrap();
}
