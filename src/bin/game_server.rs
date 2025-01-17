use rusty_kingdom::handler;
use std::io::Error;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

fn fortress_router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(handler::game::fortress_get_all))
        .routes(routes!(handler::game::fortress_new))
        .routes(routes!(handler::game::fortress_get))
        .routes(routes!(handler::game::fortress_delete))
        .routes(routes!(handler::game::fortress_gold_get))
        .routes(routes!(handler::game::fortress_gold_collect))
        .routes(routes!(handler::game::fortress_food_get))
        .routes(routes!(handler::game::fortress_food_collect))
        .routes(routes!(handler::game::fortress_wood_get))
        .routes(routes!(handler::game::fortress_wood_collect))
        .routes(routes!(handler::game::fortress_energy_get))
        .routes(routes!(handler::game::fortress_energy_collect))
        .routes(routes!(handler::game::fortress_building_get_all))
    // with state ?
}

fn building_router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(handler::game::building_get_all))
        .routes(routes!(handler::game::building_get))
        .routes(routes!(handler::game::building_improve))
    // with state ?
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    #[derive(OpenApi)]
    #[openapi()]
    struct ApiDoc;

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/v1/fortress", fortress_router())
        .nest("/api/v1/building", building_router())
        .split_for_parts();

    let router = router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api));
    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    let listener = TcpListener::bind(&address).await?;
    axum::serve(listener, router.into_make_service()).await
}
