use crate::game::{
    CollectFortressRequest, CreateFortressRequest, DeleteFortressRequest, GetBuildingRequest,
    GetFortressEnergyRequest, GetFortressFoodRequest, GetFortressGoldRequest, GetFortressRequest,
    GetFortressWoodRequest, GetImproveBuildingCostsRequest, ImproveBuildingRequest,
    ListBuildingsByFortressRequest, ListBuildingsRequest, ListFortressesRequest,
    building_service_client::BuildingServiceClient, fortress_service_client::FortressServiceClient,
};
use clap::{Command, CommandFactory, Parser, Subcommand};
use clap_complete::{Generator, Shell, generate};
use serde_json::json;
use std::io;
use tonic::transport::Channel;

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

#[derive(Parser, Debug)]
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
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
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

#[derive(Subcommand, Debug, Clone)]
enum BuildingCommands {
    GetAll,
    Get { building_id: i32 },
    Improve { building_id: i32 },
    GetImproveCosts { building_id: i32 },
}

#[derive(Subcommand, Debug, Clone)]
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

fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
    generate(
        generator,
        cmd,
        cmd.get_name().to_string(),
        &mut io::stdout(),
    );
}

async fn handle_fortress(
    fortress_client: &mut FortressServiceClient<Channel>,
    building_client: &mut BuildingServiceClient<Channel>,
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
            println!("{}{}", json!(response.fortress), json!(response.buildings));
        }
        FortressCommands::Get { fortress_id } => {
            let response = fortress_client
                .get_fortress(GetFortressRequest { id: fortress_id })
                .await?
                .into_inner();
            let fortress = response
                .fortress
                .ok_or_else(|| "fortress not found".to_string())?;
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
                .collect_fortress_gold(CollectFortressRequest { id: fortress_id })
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
                .collect_fortress_food(CollectFortressRequest { id: fortress_id })
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
                .collect_fortress_wood(CollectFortressRequest { id: fortress_id })
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
                .collect_fortress_energy(CollectFortressRequest { id: fortress_id })
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
    building_client: &mut BuildingServiceClient<Channel>,
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
            println!("{}{}", json!(response.fortress), json!(response.building));
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
    fortress_client: &mut FortressServiceClient<Channel>,
    size: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let fortress = fortress_client
        .create_fortress(CreateFortressRequest {})
        .await?
        .into_inner()
        .fortress
        .ok_or_else(|| "fortress not found".to_string())?;
    let request = CollectFortressRequest { id: fortress.id };
    for _ in 0..size {
        let _ = fortress_client.collect_fortress_gold(request).await?;
    }
    let response = fortress_client
        .get_fortress_gold(GetFortressGoldRequest { id: fortress.id })
        .await?
        .into_inner();
    println!("gold: {}", response.gold);
    let _ = fortress_client
        .delete_fortress(DeleteFortressRequest { id: fortress.id })
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.cmd {
        Commands::Fortress { cmd } => {
            let mut game_building_client = BuildingServiceClient::connect(args.url.clone()).await?;
            let mut game_fortress_client = FortressServiceClient::connect(args.url.clone()).await?;

            handle_fortress(&mut game_fortress_client, &mut game_building_client, cmd).await?;
        }
        Commands::Building { cmd } => {
            let mut game_building_client = BuildingServiceClient::connect(args.url.clone()).await?;

            handle_building(&mut game_building_client, cmd).await?;
        }
        Commands::Bench { size } => {
            let mut game_fortress_client = FortressServiceClient::connect(args.url.clone()).await?;

            handle_bench(&mut game_fortress_client, size).await?;
        }
        Commands::Completions { shell } => {
            let mut cmd = Args::command();

            print_completions(shell, &mut cmd);
        }
    }
    Ok(())
}
