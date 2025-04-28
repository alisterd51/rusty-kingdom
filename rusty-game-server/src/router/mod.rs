use crate::{ApiState, handler};
use axum::{Router, routing::get};

fn fortress() -> Router<ApiState> {
    Router::new()
        .route("/", get(handler::fortress::get_all))
        .route("/new", get(handler::fortress::new))
        .route(
            "/{fortress_id}",
            get(handler::fortress::get).delete(handler::fortress::delete),
        )
        .route("/{fortress_id}/gold", get(handler::fortress::gold_get))
        .route(
            "/{fortress_id}/gold/collect",
            get(handler::fortress::gold_collect),
        )
        .route("/{fortress_id}/food", get(handler::fortress::food_get))
        .route(
            "/{fortress_id}/food/collect",
            get(handler::fortress::food_collect),
        )
        .route("/{fortress_id}/wood", get(handler::fortress::wood_get))
        .route(
            "/{fortress_id}/wood/collect",
            get(handler::fortress::wood_collect),
        )
        .route("/{fortress_id}/energy", get(handler::fortress::energy_get))
        .route(
            "/{fortress_id}/energy/collect",
            get(handler::fortress::energy_collect),
        )
        .route(
            "/{fortress_id}/building",
            get(handler::fortress::building_get_all),
        )
}

fn building() -> Router<ApiState> {
    Router::new()
        .route("/", get(handler::building::get_all))
        .route("/{building_id}", get(handler::building::get))
        .route("/{building_id}/improve", get(handler::building::improve))
        .route(
            "/{building_id}/improve/costs",
            get(handler::building::improve_costs),
        )
}

fn api() -> Router<ApiState> {
    Router::new()
        .nest("/fortress", fortress())
        .nest("/building", building())
}

pub fn router() -> Router {
    let state = ApiState {
        api_url: std::env::var("CRUD_SERVER_URL").unwrap(),
        client: reqwest::Client::new(),
    };
    Router::new()
        .nest("/api", api())
        .with_state(state)
        .route("/health", get(handler::health))
}
