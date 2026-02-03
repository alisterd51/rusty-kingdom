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
}

pub mod service;

use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use pb::crud::v1::{
    building_service_server::BuildingServiceServer, fortress_service_server::FortressServiceServer,
};
use service::{MyBuildingService, MyFortressService};
use std::sync::Arc;
use tokio::signal::unix::{SignalKind, signal};
use tonic::transport::Server;
use tracing::{error, info, warn};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

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
    let database_url = std::env::var("DATABASE_URL").map_err(|e| format!("DATABASE_URL {e}"))?;
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager)?;
    let pool = Arc::new(pool);
    let building_service = MyBuildingService::new(pool.clone());
    let fortress_service = MyFortressService::new(pool);

    info!("Listening on {addr}");

    Server::builder()
        .add_service(BuildingServiceServer::new(building_service))
        .add_service(FortressServiceServer::new(fortress_service))
        .serve_with_shutdown(addr, shutdown_signal())
        .await?;

    Ok(())
}
