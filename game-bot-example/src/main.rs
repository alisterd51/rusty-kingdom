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
use serde_json::{Value, json};
use std::fs;
use tonic::{
    Status,
    metadata::MetadataValue,
    service::Interceptor,
    transport::{Channel, ClientTlsConfig},
};

const SERVER_URL: &str = "https://rusty.anclarma.fr";
const FORTRESS_ID: i32 = 42;
const AUTH_URL: &str = "https://auth.rusty.anclarma.fr";
const CLIENT_ID: &str = "rusty-client";
// const SERVER_URL: &str = "http://localhost:8080";
// const FORTRESS_ID: i32 = 1;
// const AUTH_URL: &str = "http://localhost:8082";
// const CLIENT_ID: &str = "rusty-client";

#[derive(Clone)]
struct AuthInterceptor {
    token: String,
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut req: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        let meta: MetadataValue<_> = format!("Bearer {}", self.token)
            .parse()
            .map_err(|_| Status::unauthenticated("Invalid token"))?;
        req.metadata_mut().insert("authorization", meta);
        Ok(req)
    }
}

async fn refresh_session(
    auth_url: &str,
    client_id: &str,
    refresh_token: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{auth_url}/auth/v1/oidc/token"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!(
            "grant_type=refresh_token&client_id={client_id}&refresh_token={refresh_token}"
        ))
        .send()
        .await?;

    if !res.status().is_success() {
        return Err(format!("Rauthy error ({}) : {}", res.status(), res.text().await?).into());
    }

    let new_session: Value = res.json().await?;
    fs::write(".bot_token", serde_json::to_string_pretty(&new_session)?)?;
    Ok(new_session)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(".bot_token").map_err(
        |_| "The .bot_token file could not be found. Generate one using the game-client CLI.",
    )?;

    let mut session: Value = if content.trim().starts_with('{') {
        serde_json::from_str(&content)?
    } else {
        json!({ "refresh_token": content.trim() })
    };

    let mut endpoint = Channel::from_static(SERVER_URL);
    if SERVER_URL.starts_with("https://") {
        endpoint = endpoint.tls_config(ClientTlsConfig::new().with_native_roots())?;
    }
    let channel = endpoint.connect().await?;

    println!("Connected bot!");

    let mut i = 0;
    while i < 1000 {
        let mut access_token = session["access_token"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        let refresh_token = session["refresh_token"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        if access_token.is_empty() {
            println!("Initializing the session...");
            session = refresh_session(AUTH_URL, CLIENT_ID, &refresh_token).await?;
            access_token = session["access_token"]
                .as_str()
                .unwrap_or_default()
                .to_string();
        }

        let interceptor = AuthInterceptor {
            token: access_token,
        };
        let mut fortress_client =
            FortressServiceClient::with_interceptor(channel.clone(), interceptor);

        match fortress_client
            .collect_fortress_gold(CollectFortressGoldRequest { id: FORTRESS_ID })
            .await
        {
            Ok(_) => {
                let _ = fortress_client
                    .collect_fortress_food(CollectFortressFoodRequest { id: FORTRESS_ID })
                    .await;
                let _ = fortress_client
                    .collect_fortress_wood(CollectFortressWoodRequest { id: FORTRESS_ID })
                    .await;
                let _ = fortress_client
                    .collect_fortress_energy(CollectFortressEnergyRequest { id: FORTRESS_ID })
                    .await;

                i += 1;
                if i % 100 == 0 {
                    println!("{i} cycles completed...");
                }
            }
            Err(e) if e.code() == tonic::Code::Unauthenticated => {
                println!("The access token has expired. Silent refresh...");
                session = refresh_session(AUTH_URL, CLIENT_ID, &refresh_token).await?;
            }
            Err(e) => {
                println!("Unexpected error: {e:?}");
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }

    println!("Collection complete!");
    Ok(())
}
