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

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};
use pb::game::v1::{
    CollectFortressEnergyRequest, CollectFortressFoodRequest, CollectFortressGoldRequest,
    CollectFortressWoodRequest, CreateFortressRequest, DeleteFortressRequest, GetBuildingRequest,
    GetFortressEnergyRequest, GetFortressFoodRequest, GetFortressGoldRequest, GetFortressRequest,
    GetFortressWoodRequest, GetImproveBuildingCostsRequest, ImproveBuildingRequest,
    ListBuildingsByFortressRequest, ListBuildingsRequest, ListFortressesRequest,
    building_service_client::BuildingServiceClient, fortress_service_client::FortressServiceClient,
};
use serde_json::json;
use std::{fs, io, time::Duration};
use tonic::{
    Status,
    metadata::MetadataValue,
    service::{Interceptor, interceptor::InterceptedService},
    transport::{Channel, ClientTlsConfig},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,

    #[arg(
        short,
        long,
        value_name = "SERVER_URL",
        default_value = "https://rusty.anclarma.fr"
    )]
    url: String,

    #[arg(
        long,
        value_name = "AUTH_URL",
        default_value = "https://auth.rusty.anclarma.fr"
    )]
    auth_url: String,

    #[arg(long, help = "Disable authentication (Guest Mode)")]
    no_auth: bool,
}

#[derive(Subcommand, Clone)]
enum Commands {
    Login,
    Fortress {
        #[command(subcommand)]
        cmd: FortressCommands,
    },
    Building {
        #[command(subcommand)]
        cmd: BuildingCommands,
    },
    Bench {
        size: usize,
    },
    Completions {
        shell: Shell,
    },
}

#[derive(Subcommand, Clone)]
enum BuildingCommands {
    GetAll,
    Get { building_id: i32 },
    Improve { building_id: i32 },
    GetImproveCosts { building_id: i32 },
}

#[derive(Subcommand, Clone)]
enum FortressCommands {
    GetAll,
    New,
    Get { fortress_id: i32 },
    Delete { fortress_id: i32 },
    GetGold { fortress_id: i32 },
    CollectGold { fortress_id: i32 },
    GetFood { fortress_id: i32 },
    CollectFood { fortress_id: i32 },
    GetWood { fortress_id: i32 },
    CollectWood { fortress_id: i32 },
    GetEnergy { fortress_id: i32 },
    CollectEnergy { fortress_id: i32 },
    GetAllBuildings { fortress_id: i32 },
}

#[derive(Clone)]
struct AuthInterceptor {
    token: String,
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        if self.token.is_empty() {
            return Ok(request);
        }
        let bearer = format!("Bearer {}", self.token);
        MetadataValue::try_from(&bearer).map_or_else(
            |_| Err(Status::unauthenticated("Invalid access token")),
            |meta| {
                request.metadata_mut().insert("authorization", meta);
                Ok(request)
            },
        )
    }
}

async fn handle_fortress(
    fortress_client: &mut FortressServiceClient<InterceptedService<Channel, AuthInterceptor>>,
    building_client: &mut BuildingServiceClient<InterceptedService<Channel, AuthInterceptor>>,
    cmd: FortressCommands,
) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        FortressCommands::GetAll => {
            let response = fortress_client
                .list_fortresses(ListFortressesRequest {})
                .await?
                .into_inner();
            println!("{}", json!(response.fortresses));
        }
        FortressCommands::New => {
            let response = fortress_client
                .create_fortress(CreateFortressRequest {})
                .await?
                .into_inner();
            println!(
                "{}",
                json!({"fortress": response.fortress, "buildings": response.buildings})
            );
        }
        FortressCommands::Get { fortress_id } => {
            let response = fortress_client
                .get_fortress(GetFortressRequest { id: fortress_id })
                .await?
                .into_inner();
            let fortress = response.fortress.ok_or("fortress not found")?;
            println!("{}", json!(fortress));
        }
        FortressCommands::Delete { fortress_id } => {
            let response = fortress_client
                .delete_fortress(DeleteFortressRequest { id: fortress_id })
                .await?
                .into_inner();
            println!("{}", json!(response.success));
        }
        FortressCommands::GetGold { fortress_id } => {
            let response = fortress_client
                .get_fortress_gold(GetFortressGoldRequest { id: fortress_id })
                .await?
                .into_inner();
            println!("{}", json!(response.gold));
        }
        FortressCommands::CollectGold { fortress_id } => {
            let response = fortress_client
                .collect_fortress_gold(CollectFortressGoldRequest { id: fortress_id })
                .await?
                .into_inner();
            println!("{}", json!(response.fortress));
        }
        FortressCommands::GetFood { fortress_id } => {
            let response = fortress_client
                .get_fortress_food(GetFortressFoodRequest { id: fortress_id })
                .await?
                .into_inner();
            println!("{}", json!(response.food));
        }
        FortressCommands::CollectFood { fortress_id } => {
            let response = fortress_client
                .collect_fortress_food(CollectFortressFoodRequest { id: fortress_id })
                .await?
                .into_inner();
            println!("{}", json!(response.fortress));
        }
        FortressCommands::GetWood { fortress_id } => {
            let response = fortress_client
                .get_fortress_wood(GetFortressWoodRequest { id: fortress_id })
                .await?
                .into_inner();
            println!("{}", json!(response.wood));
        }
        FortressCommands::CollectWood { fortress_id } => {
            let response = fortress_client
                .collect_fortress_wood(CollectFortressWoodRequest { id: fortress_id })
                .await?
                .into_inner();
            println!("{}", json!(response.fortress));
        }
        FortressCommands::GetEnergy { fortress_id } => {
            let response = fortress_client
                .get_fortress_energy(GetFortressEnergyRequest { id: fortress_id })
                .await?
                .into_inner();
            println!("{}", json!(response.energy));
        }
        FortressCommands::CollectEnergy { fortress_id } => {
            let response = fortress_client
                .collect_fortress_energy(CollectFortressEnergyRequest { id: fortress_id })
                .await?
                .into_inner();
            println!("{}", json!(response.fortress));
        }
        FortressCommands::GetAllBuildings { fortress_id } => {
            let response = building_client
                .list_buildings_by_fortress(ListBuildingsByFortressRequest { fortress_id })
                .await?
                .into_inner();
            println!("{}", json!(response.buildings));
        }
    }
    Ok(())
}

async fn handle_building(
    building_client: &mut BuildingServiceClient<InterceptedService<Channel, AuthInterceptor>>,
    cmd: BuildingCommands,
) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        BuildingCommands::GetAll => {
            let response = building_client
                .list_buildings(ListBuildingsRequest {})
                .await?
                .into_inner();
            println!("{}", json!(response.buildings));
        }
        BuildingCommands::Get { building_id } => {
            let response = building_client
                .get_building(GetBuildingRequest { id: building_id })
                .await?
                .into_inner();
            println!("{}", json!(response.building));
        }
        BuildingCommands::Improve { building_id } => {
            let response = building_client
                .improve_building(ImproveBuildingRequest { id: building_id })
                .await?
                .into_inner();
            println!(
                "{}",
                json!({"fortress": response.fortress, "building": response.building})
            );
        }
        BuildingCommands::GetImproveCosts { building_id } => {
            let response = building_client
                .get_improve_building_costs(GetImproveBuildingCostsRequest { id: building_id })
                .await?
                .into_inner();
            println!("{}", json!(response.costs));
        }
    }
    Ok(())
}

async fn handle_bench(
    fortress_client: &mut FortressServiceClient<InterceptedService<Channel, AuthInterceptor>>,
    size: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let fortress = fortress_client
        .create_fortress(CreateFortressRequest {})
        .await?
        .into_inner()
        .fortress
        .ok_or("fortress not found")?;
    let request = CollectFortressGoldRequest { id: fortress.id };
    for _ in 0..size {
        fortress_client.collect_fortress_gold(request).await?;
    }
    let response = fortress_client
        .get_fortress_gold(GetFortressGoldRequest { id: fortress.id })
        .await?
        .into_inner();
    println!("gold: {}", response.gold);
    fortress_client
        .delete_fortress(DeleteFortressRequest { id: fortress.id })
        .await?;
    Ok(())
}

async fn handle_login(auth_url: &str, client_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let device_auth_url = format!("{auth_url}/auth/v1/oidc/device");
    let token_url = format!("{auth_url}/auth/v1/oidc/token");

    println!("Initializing the connection...");

    let auth_payload = format!("client_id={client_id}&scope=openid%20profile%20email");

    let res = client
        .post(&device_auth_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(auth_payload)
        .send()
        .await?;

    if !res.status().is_success() {
        let status = res.status();
        let error_body = res.text().await.unwrap_or_default();
        return Err(format!("Erreur API Rauthy ({status}) : {error_body}").into());
    }

    let res_json = res.json::<serde_json::Value>().await?;

    let device_code = res_json["device_code"].as_str().ok_or("No device_code")?;
    let user_code = res_json["user_code"].as_str().ok_or("No user_code")?;
    let verification_uri = res_json["verification_uri_complete"]
        .as_str()
        .or_else(|| res_json["verification_uri"].as_str())
        .ok_or("No verification_uri")?;
    let interval = res_json["interval"].as_u64().unwrap_or(5);

    println!("====================================================");
    println!("Please open this URL in your browser :");
    println!("{verification_uri}");
    println!("Verify that the code matches : {user_code}");
    println!("Waiting for your validation... (do not close this terminal)");
    println!("====================================================");

    loop {
        tokio::time::sleep(Duration::from_secs(interval)).await;

        let token_payload = format!(
            "grant_type=urn:ietf:params:oauth:grant-type:device_code&client_id={client_id}&device_code={device_code}"
        );

        let token_res = client
            .post(&token_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(token_payload)
            .send()
            .await?;

        if token_res.status().is_success() {
            let token_json: serde_json::Value = token_res.json().await?;

            fs::write(
                ".rusty_token",
                token_json["access_token"]
                    .as_str()
                    .ok_or("No access_token")?,
            )?;
            fs::write(".bot_token", serde_json::to_string_pretty(&token_json)?)?;

            println!("Connection successful! CLI and Bot tokens saved locally.");
            break;
        }
        let err = token_res.json::<serde_json::Value>().await?;
        let error_code = err["error"].as_str().unwrap_or("");
        if error_code != "authorization_pending" && error_code != "slow_down" {
            return Err(format!("Authentication failed: {error_code}").into());
        }
    }

    Ok(())
}

// TODO: Should we use `keyring` instead?
fn get_local_token() -> Result<String, Box<dyn std::error::Error>> {
    fs::read_to_string(".rusty_token")
        .map_err(|_| "Not logged in. Please launch 'game-client login' first.".into())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.cmd {
        Commands::Login => {
            handle_login(&args.auth_url, "rusty-client").await?;
            return Ok(());
        }
        Commands::Completions { shell } => {
            let mut cmd = Args::command();
            generate(shell, &mut cmd, "game-client", &mut io::stdout());
            return Ok(());
        }
        _ => {}
    }

    let token = if args.no_auth {
        String::new()
    } else {
        get_local_token()?
    };
    let interceptor = AuthInterceptor { token };
    let mut endpoint = Channel::from_shared(args.url.clone())?;

    if args.url.starts_with("https://") {
        let tls = ClientTlsConfig::new().with_native_roots();
        endpoint = endpoint.tls_config(tls)?;
    }

    let channel = endpoint.connect().await?;

    let mut game_building_client =
        BuildingServiceClient::with_interceptor(channel.clone(), interceptor.clone());
    let mut game_fortress_client = FortressServiceClient::with_interceptor(channel, interceptor);

    match args.cmd {
        Commands::Fortress { cmd } => {
            handle_fortress(&mut game_fortress_client, &mut game_building_client, cmd).await?;
        }
        Commands::Building { cmd } => {
            handle_building(&mut game_building_client, cmd).await?;
        }
        Commands::Bench { size } => {
            handle_bench(&mut game_fortress_client, size).await?;
        }
        _ => unreachable!(),
    }
    Ok(())
}
