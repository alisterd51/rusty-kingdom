use crate::game::{
    CollectFortressEnergyRequest, CollectFortressFoodRequest, CollectFortressGoldRequest,
    CollectFortressWoodRequest, CreateFortressRequest, DeleteFortressRequest, GetBuildingRequest,
    GetFortressEnergyRequest, GetFortressFoodRequest, GetFortressGoldRequest, GetFortressRequest,
    GetFortressWoodRequest, GetImproveBuildingCostsRequest, ImproveBuildingRequest,
    ListBuildingsByFortressRequest, ListBuildingsRequest, ListFortressesRequest,
    building_service_client::BuildingServiceClient, fortress_service_client::FortressServiceClient,
};
use clap::{Command, CommandFactory, Parser, Subcommand};
use clap_complete::{Generator, Shell, generate};
use serde_json::json;
use std::io;

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
        default_value = "http://localhost:8080"
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

#[allow(clippy::too_many_lines)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let game_server_url = "http://localhost:8080".to_string();
    let game_building_client = BuildingServiceClient::connect(game_server_url.clone()).await?;
    let game_fortress_client = FortressServiceClient::connect(game_server_url.clone()).await?;

    match args.cmd {
        Commands::Fortress { cmd } => match cmd {
            FortressCommands::GetAll => {
                let request = ListFortressesRequest {};
                let fortresses = game_fortress_client
                    .clone()
                    .list_fortresses(request)
                    .await?
                    .into_inner()
                    .fortresses;
                println!("{}", json!(fortresses));
            }
            FortressCommands::New => {
                let request = CreateFortressRequest {};
                let response = game_fortress_client
                    .clone()
                    .create_fortress(request)
                    .await?
                    .into_inner();
                let fortress = response.fortress;
                let buildings = response.buildings;

                println!("{}{}", json!(fortress), json!(buildings));
            }
            FortressCommands::Get { fortress_id } => {
                let request = GetFortressRequest { id: fortress_id };

                let fortress = game_fortress_client
                    .clone()
                    .get_fortress(request)
                    .await?
                    .into_inner()
                    .fortress
                    .ok_or_else(|| "fortress not found".to_string())?;

                println!("{}", json!(fortress));
            }
            FortressCommands::Delete { fortress_id } => {
                let request = DeleteFortressRequest { id: fortress_id };

                let success = game_fortress_client
                    .clone()
                    .delete_fortress(request)
                    .await?
                    .into_inner()
                    .success;

                println!("{}", json!(success));
            }
            FortressCommands::GetGold { fortress_id } => {
                let request = GetFortressGoldRequest { id: fortress_id };
                let response = game_fortress_client
                    .clone()
                    .get_fortress_gold(request)
                    .await?
                    .into_inner();
                println!("{}", json!(response.gold));
            }
            FortressCommands::CollectGold { fortress_id } => {
                let request = CollectFortressGoldRequest { id: fortress_id };
                let response = game_fortress_client
                    .clone()
                    .collect_fortress_gold(request)
                    .await?
                    .into_inner();
                println!("{}", json!(response.fortress));
            }
            FortressCommands::GetFood { fortress_id } => {
                let request = GetFortressFoodRequest { id: fortress_id };
                let response = game_fortress_client
                    .clone()
                    .get_fortress_food(request)
                    .await?
                    .into_inner();
                println!("{}", json!(response.food));
            }
            FortressCommands::CollectFood { fortress_id } => {
                let request = CollectFortressFoodRequest { id: fortress_id };
                let response = game_fortress_client
                    .clone()
                    .collect_fortress_food(request)
                    .await?
                    .into_inner();
                println!("{}", json!(response.fortress));
            }
            FortressCommands::GetWood { fortress_id } => {
                let request = GetFortressWoodRequest { id: fortress_id };
                let response = game_fortress_client
                    .clone()
                    .get_fortress_wood(request)
                    .await?
                    .into_inner();
                println!("{}", json!(response.wood));
            }
            FortressCommands::CollectWood { fortress_id } => {
                let request = CollectFortressWoodRequest { id: fortress_id };
                let response = game_fortress_client
                    .clone()
                    .collect_fortress_wood(request)
                    .await?
                    .into_inner();
                println!("{}", json!(response.fortress));
            }
            FortressCommands::GetEnergy { fortress_id } => {
                let request = GetFortressEnergyRequest { id: fortress_id };
                let response = game_fortress_client
                    .clone()
                    .get_fortress_energy(request)
                    .await?
                    .into_inner();
                println!("{}", json!(response.energy));
            }
            FortressCommands::CollectEnergy { fortress_id } => {
                let request = CollectFortressEnergyRequest { id: fortress_id };
                let response = game_fortress_client
                    .clone()
                    .collect_fortress_energy(request)
                    .await?
                    .into_inner();
                println!("{}", json!(response.fortress));
            }
            FortressCommands::GetAllBuildings { fortress_id } => {
                let request = ListBuildingsByFortressRequest { fortress_id };
                let response = game_building_client
                    .clone()
                    .list_buildings_by_fortress(request)
                    .await?
                    .into_inner();
                println!("{}", json!(response.buildings));
            }
        },
        Commands::Building { cmd } => match cmd {
            BuildingCommands::GetAll => {
                let request = ListBuildingsRequest {};
                let response = game_building_client
                    .clone()
                    .list_buildings(request)
                    .await?
                    .into_inner();
                println!("{}", json!(response.buildings));
            }
            BuildingCommands::Get { building_id } => {
                let request = GetBuildingRequest { id: building_id };
                let response = game_building_client
                    .clone()
                    .get_building(request)
                    .await?
                    .into_inner();
                println!("{}", json!(response.building));
            }
            BuildingCommands::Improve { building_id } => {
                let request = ImproveBuildingRequest { id: building_id };
                let response = game_building_client
                    .clone()
                    .improve_building(request)
                    .await?
                    .into_inner();
                println!("{}{}", json!(response.fortress), json!(response.building));
            }
            BuildingCommands::GetImproveCosts { building_id } => {
                let request = GetImproveBuildingCostsRequest { id: building_id };
                let costs = game_building_client
                    .clone()
                    .get_improve_building_costs(request)
                    .await?
                    .into_inner()
                    .costs
                    .ok_or_else(|| "costs not found".to_string())?;
                println!("{}", json!(costs));
            }
        },
        Commands::Bench { size } => {
            let request = CreateFortressRequest {};
            let fortress = game_fortress_client
                .clone()
                .create_fortress(request)
                .await?
                .into_inner()
                .fortress
                .ok_or_else(|| "fortress not found".to_string())?;

            let request = CollectFortressGoldRequest { id: fortress.id };
            for _ in 0..size {
                let _ = game_fortress_client
                    .clone()
                    .collect_fortress_gold(request)
                    .await?;
            }

            let request = GetFortressGoldRequest { id: fortress.id };
            let gold = game_fortress_client
                .clone()
                .get_fortress_gold(request)
                .await?
                .into_inner()
                .gold;
            println!("gold: {gold}");

            let request = DeleteFortressRequest { id: fortress.id };
            let _ = game_fortress_client
                .clone()
                .delete_fortress(request)
                .await?;
        }
        Commands::Completions { shell } => {
            let mut cmd = Args::command();
            print_completions(shell, &mut cmd);
        }
    }
    Ok(())
}
