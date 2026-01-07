pub mod service;

use crate::{
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
use tokio::signal::unix::{SignalKind, signal};
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::cors::CorsLayer;

#[allow(clippy::pedantic)]
#[allow(clippy::nursery)]
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::]:3000".parse()?;
    let crud_server_url =
        std::env::var("CRUD_SERVER_URL").map_err(|e| format!("CRUD_SERVER_URL {e}"))?;
    let crud_building_client = BuildingServiceClient::connect(crud_server_url.clone()).await?;
    let crud_fortress_client = FortressServiceClient::connect(crud_server_url.clone()).await?;

    let building_service =
        MyBuildingService::new(crud_building_client.clone(), crud_fortress_client.clone());
    let fortress_service =
        MyFortressService::new(crud_building_client.clone(), crud_fortress_client.clone());

    let shutdown_signal = async {
        let mut sigterm = signal(SignalKind::terminate()).unwrap();
        let mut sigint = signal(SignalKind::interrupt()).unwrap();

        tokio::select! {
            _ = sigterm.recv() => {}
            _ = sigint.recv() => {}
        }
    };

    Server::builder()
        .accept_http1(true)
        .layer(CorsLayer::permissive())
        .layer(GrpcWebLayer::new())
        .add_service(BuildingServiceServer::new(building_service))
        .add_service(FortressServiceServer::new(fortress_service))
        .serve_with_shutdown(addr, shutdown_signal)
        .await?;

    Ok(())
}
