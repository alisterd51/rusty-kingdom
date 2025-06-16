pub mod service;

use crate::{
    crud::{
        building_service_server::BuildingServiceServer,
        fortress_service_server::FortressServiceServer,
    },
    service::{MyBuildingService, MyFortressService},
};
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use std::sync::Arc;
use tokio::signal::unix::{SignalKind, signal};
use tonic::transport::Server;

#[allow(clippy::pedantic)]
#[allow(clippy::nursery)]
pub mod common {
    tonic::include_proto!("common");
}

#[allow(clippy::pedantic)]
#[allow(clippy::nursery)]
pub mod crud {
    tonic::include_proto!("crud");
}

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::]:3000".parse()?;
    let database_url = std::env::var("DATABASE_URL").map_err(|e| format!("DATABASE_URL {e}"))?;
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager)?;
    let pool = Arc::new(pool);
    let building_service = MyBuildingService::new(pool.clone());
    let fortress_service = MyFortressService::new(pool.clone());

    let shutdown_signal = async {
        let mut sigterm = signal(SignalKind::terminate()).unwrap();
        let mut sigint = signal(SignalKind::interrupt()).unwrap();

        tokio::select! {
            _ = sigterm.recv() => {
            }
            _ = sigint.recv() => {
            }
        }
    };

    Server::builder()
        .add_service(BuildingServiceServer::new(building_service))
        .add_service(FortressServiceServer::new(fortress_service))
        .serve_with_shutdown(addr, shutdown_signal)
        .await?;

    Ok(())
}
