use crate::game::{CollectFortressRequest, fortress_service_client::FortressServiceClient};

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut fortress_client = FortressServiceClient::connect(SERVER_URL).await?;
    let collect_request = CollectFortressRequest { id: FORTRESS_ID };
    for _ in 0..1000 {
        let _ = fortress_client
            .collect_fortress_gold(collect_request)
            .await?;
        let _ = fortress_client
            .collect_fortress_food(collect_request)
            .await?;
        let _ = fortress_client
            .collect_fortress_wood(collect_request)
            .await?;
        let _ = fortress_client
            .collect_fortress_energy(collect_request)
            .await?;
    }
    Ok(())
}
