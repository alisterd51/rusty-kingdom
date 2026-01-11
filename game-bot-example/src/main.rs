#![allow(clippy::similar_names)]

#[allow(clippy::pedantic, clippy::nursery)]
pub mod pb {
    pub mod common {
        pub mod v1 {
            tonic::include_proto!("common.v1");
        }
    }
    pub mod game {
        pub mod v1 {
            tonic::include_proto!("game.v1");
        }
    }
}

use pb::game::v1::{
    CollectFortressEnergyRequest, CollectFortressFoodRequest, CollectFortressGoldRequest,
    CollectFortressWoodRequest, fortress_service_client::FortressServiceClient,
};

const SERVER_URL: &str = "https://rusty.anclarma.fr";
const FORTRESS_ID: i32 = 42;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut fortress_client = FortressServiceClient::connect(SERVER_URL).await?;
    let collect_request_gold = CollectFortressGoldRequest { id: FORTRESS_ID };
    let collect_request_food = CollectFortressFoodRequest { id: FORTRESS_ID };
    let collect_request_wood = CollectFortressWoodRequest { id: FORTRESS_ID };
    let collect_request_energy = CollectFortressEnergyRequest { id: FORTRESS_ID };
    for _ in 0..1000 {
        fortress_client
            .collect_fortress_gold(collect_request_gold)
            .await?;
        fortress_client
            .collect_fortress_food(collect_request_food)
            .await?;
        fortress_client
            .collect_fortress_wood(collect_request_wood)
            .await?;
        fortress_client
            .collect_fortress_energy(collect_request_energy)
            .await?;
    }
    Ok(())
}
