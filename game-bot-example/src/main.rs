use crate::game::{
    CollectFortressEnergyRequest, CollectFortressFoodRequest, CollectFortressGoldRequest,
    CollectFortressWoodRequest, fortress_service_client::FortressServiceClient,
};

#[allow(clippy::pedantic)]
#[allow(clippy::nursery)]
pub mod common {
    tonic::include_proto!("common");
}

#[allow(clippy::pedantic)]
#[allow(clippy::nursery)]
pub mod game {
    tonic::include_proto!("game");
}

const SERVER_URL: &str = "https://rusty.anclarma.fr";
const FORTRESS_ID: i32 = 42;

#[allow(clippy::similar_names)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut fortress_client = FortressServiceClient::connect(SERVER_URL).await?;
    let collect_gold_request = CollectFortressGoldRequest { id: FORTRESS_ID };
    let collect_food_request = CollectFortressFoodRequest { id: FORTRESS_ID };
    let collect_wood_request = CollectFortressWoodRequest { id: FORTRESS_ID };
    let collect_energy_request = CollectFortressEnergyRequest { id: FORTRESS_ID };
    for _ in 0..1000 {
        let _ = fortress_client
            .collect_fortress_gold(collect_gold_request)
            .await?;
        let _ = fortress_client
            .collect_fortress_food(collect_food_request)
            .await?;
        let _ = fortress_client
            .collect_fortress_wood(collect_wood_request)
            .await?;
        let _ = fortress_client
            .collect_fortress_energy(collect_energy_request)
            .await?;
    }
    Ok(())
}
