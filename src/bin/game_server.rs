use axum::routing::{delete, get};
use rusty_kingdom::handler;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/api/fortress", get(handler::game::fortress_get_all))
        .route("/api/fortress/new", get(handler::game::fortress_new))
        .route(
            "/api/fortress/:fortress_id",
            get(handler::game::fortress_get),
        )
        .route(
            "/api/fortress/:fortress_id",
            delete(handler::game::fortress_delete),
        )
        .route(
            "/api/fortress/:fortress_id/gold",
            get(handler::game::fortress_gold_get),
        )
        .route(
            "/api/fortress/:fortress_id/gold/collect",
            get(handler::game::fortress_gold_collect),
        )
        .route(
            "/api/fortress/:fortress_id/food",
            get(handler::game::fortress_food_get),
        )
        .route(
            "/api/fortress/:fortress_id/food/collect",
            get(handler::game::fortress_food_collect),
        )
        .route(
            "/api/fortress/:fortress_id/wood",
            get(handler::game::fortress_wood_get),
        )
        .route(
            "/api/fortress/:fortress_id/wood/collect",
            get(handler::game::fortress_wood_collect),
        )
        .route(
            "/api/fortress/:fortress_id/energy",
            get(handler::game::fortress_energy_get),
        )
        .route(
            "/api/fortress/:fortress_id/energy/collect",
            get(handler::game::fortress_energy_collect),
        )
        .route(
            "/api/fortress/:fortress_id/building",
            get(handler::game::fortress_building_get_all),
        )
        .route("/api/building", get(handler::game::building_get_all))
        .route(
            "/api/building/:building_id",
            get(handler::game::building_get),
        )
        .route(
            "/api/building/:building_id/improve",
            get(handler::game::building_improve),
        );
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let tcp_listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(tcp_listener, app).await.unwrap();
}
