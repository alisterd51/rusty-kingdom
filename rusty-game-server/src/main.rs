mod handler;

use axum::routing::get;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    api_url: String,
    client: reqwest::Client,
}

#[tokio::main]
async fn main() {
    let app_state = AppState {
        api_url: std::env::var("CRUD_SERVER_URL").unwrap(),
        client: reqwest::Client::new(),
    };
    let app = axum::Router::new()
        .route("/api/fortress", get(handler::fortress::get_all))
        .route("/api/fortress/new", get(handler::fortress::new))
        .route(
            "/api/fortress/{fortress_id}",
            get(handler::fortress::get).delete(handler::fortress::delete),
        )
        .route(
            "/api/fortress/{fortress_id}/gold",
            get(handler::fortress::gold_get),
        )
        .route(
            "/api/fortress/{fortress_id}/gold/collect",
            get(handler::fortress::gold_collect),
        )
        .route(
            "/api/fortress/{fortress_id}/food",
            get(handler::fortress::food_get),
        )
        .route(
            "/api/fortress/{fortress_id}/food/collect",
            get(handler::fortress::food_collect),
        )
        .route(
            "/api/fortress/{fortress_id}/wood",
            get(handler::fortress::wood_get),
        )
        .route(
            "/api/fortress/{fortress_id}/wood/collect",
            get(handler::fortress::wood_collect),
        )
        .route(
            "/api/fortress/{fortress_id}/energy",
            get(handler::fortress::energy_get),
        )
        .route(
            "/api/fortress/{fortress_id}/energy/collect",
            get(handler::fortress::energy_collect),
        )
        .route(
            "/api/fortress/{fortress_id}/building",
            get(handler::fortress::building_get_all),
        )
        .route("/api/building", get(handler::building::get_all))
        .route("/api/building/{building_id}", get(handler::building::get))
        .route(
            "/api/building/{building_id}/improve",
            get(handler::building::improve),
        )
        .route(
            "/api/building/{building_id}/improve/costs",
            get(handler::building::improve_costs),
        )
        .with_state(app_state);
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    let tcp_listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(tcp_listener, app).await.unwrap();
}
