pub mod auth;
pub mod service;

use crate::{
    auth::AuthInterceptor,
    pb::{
        crud::v1::{
            building_service_client::BuildingServiceClient,
            fortress_service_client::FortressServiceClient,
        },
        game::v1::{
            building_service_server::BuildingServiceServer,
            fortress_service_server::FortressServiceServer,
        },
    },
    service::{MyBuildingService, MyFortressService},
};
use jsonwebtoken::jwk::JwkSet;
use std::sync::Arc;
use tokio::signal::unix::{SignalKind, signal};
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::cors::CorsLayer;
use tracing::{error, info, warn};

#[allow(clippy::pedantic, clippy::nursery)]
pub mod pb {
    pub mod common {
        pub mod v1 {
            tonic::include_proto!("common.v1");
        }
    }
    pub mod crud {
        pub mod v1 {
            tonic::include_proto!("crud.v1");
        }
    }
    pub mod game {
        pub mod v1 {
            tonic::include_proto!("game.v1");
        }
    }
}

async fn shutdown_signal() {
    let sigterm = signal(SignalKind::terminate());
    let sigint = signal(SignalKind::interrupt());

    match (sigterm, sigint) {
        (Ok(mut term), Ok(mut int)) => {
            tokio::select! {
                _ = term.recv() => warn!("SIGTERM received"),
                _ = int.recv() => warn!("SIGINT received"),
            };
        }
        (Err(e), _) | (_, Err(e)) => {
            error!("Failed to setup signal handler: {e}");
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let addr = "[::]:3000".parse()?;
    let crud_server_url =
        std::env::var("CRUD_SERVER_URL").map_err(|e| format!("CRUD_SERVER_URL {e}"))?;
    let auth_url =
        std::env::var("AUTH_URL").unwrap_or_else(|_| "https://auth.rusty.anclarma.fr".to_string());
    let issuer_url = std::env::var("ISSUER_URL").unwrap_or_else(|_| auth_url.clone());

    info!("Downloading public keys from Rauthy ({auth_url})...");
    let jwks_url = format!("{auth_url}/auth/v1/oidc/certs");
    let jwks: JwkSet = reqwest::Client::new()
        .get(&jwks_url)
        .send()
        .await
        .map_err(|e| format!("Unable to reach Rauthy: {e}"))?
        .json()
        .await
        .map_err(|e| format!("JWKS parsing error: {e}"))?;

    let auth_interceptor = AuthInterceptor {
        jwks: Arc::new(jwks),
        issuer: format!("{issuer_url}/auth/v1/"),
    };

    let crud_building_client = BuildingServiceClient::connect(crud_server_url.clone()).await?;
    let crud_fortress_client = FortressServiceClient::connect(crud_server_url).await?;
    let building_service =
        MyBuildingService::new(crud_building_client.clone(), crud_fortress_client.clone());
    let fortress_service = MyFortressService::new(crud_building_client, crud_fortress_client);

    info!("Listening on {addr}");

    Server::builder()
        .accept_http1(true)
        .layer(CorsLayer::permissive())
        .layer(GrpcWebLayer::new())
        .add_service(BuildingServiceServer::with_interceptor(
            building_service,
            auth_interceptor.clone(),
        ))
        .add_service(FortressServiceServer::with_interceptor(
            fortress_service,
            auth_interceptor,
        ))
        .serve_with_shutdown(addr, shutdown_signal())
        .await?;

    Ok(())
}
