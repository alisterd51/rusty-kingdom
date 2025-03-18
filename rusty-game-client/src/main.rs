use clap::{Parser, Subcommand};
use rusty::request::game;

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
    Bench { size: usize },
    GetAllFortress,
    NewFortress,
    GetFortress { fortress_id: i32 },
    DeleteFortress { fortress_id: i32 },
    GetGoldByFortress { fortress_id: i32 },
    CollectGoldByFortress { fortress_id: i32 },
    GetFoodByFortress { fortress_id: i32 },
    CollectFoodByFortress { fortress_id: i32 },
    GetWoodByFortress { fortress_id: i32 },
    CollectWoodByFortress { fortress_id: i32 },
    GetEnergyByFortress { fortress_id: i32 },
    CollectEnergyByFortress { fortress_id: i32 },
    GetAllBuildingsByFortress { fortress_id: i32 },
    GetAllBuildings,
    GetBuilding { building_id: i32 },
    ImproveBuilding { building_id: i32 },
    GetImproveBuildingCosts { building_id: i32 },
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Args::parse();
    let client = reqwest::Client::new();

    match args.cmd {
        Commands::Bench { size } => {
            let (fortress, _buildings) = game::fortress::new(&client, &args.url).await?;
            for _ in 0..size {
                let _ = game::fortress::gold_collect(&client, &args.url, fortress.id).await;
            }
            let gold = game::fortress::gold_get(&client, &args.url, fortress.id).await?;
            println!("gold: {gold}");
            game::fortress::delete(&client, &args.url, fortress.id).await?;
        }
        Commands::GetAllFortress => {
            let fortresses = game::fortress::get_all(&client, &args.url).await?;
            println!("{fortresses:?}");
        }
        Commands::NewFortress => {
            let response = game::fortress::new(&client, &args.url).await?;
            println!("{response:?}");
        }
        Commands::GetFortress { fortress_id } => {
            let fortress = game::fortress::get(&client, &args.url, fortress_id).await?;
            println!("{fortress:?}");
        }
        Commands::DeleteFortress { fortress_id } => {
            let fortress = game::fortress::delete(&client, &args.url, fortress_id).await?;
            println!("{fortress:?}");
        }
        Commands::GetGoldByFortress { fortress_id } => {
            let gold = game::fortress::gold_get(&client, &args.url, fortress_id).await?;
            println!("{gold:?}");
        }
        Commands::CollectGoldByFortress { fortress_id } => {
            let fortress = game::fortress::gold_collect(&client, &args.url, fortress_id).await?;
            println!("{fortress:?}");
        }
        Commands::GetFoodByFortress { fortress_id } => {
            let food = game::fortress::gold_get(&client, &args.url, fortress_id).await?;
            println!("{food:?}");
        }
        Commands::CollectFoodByFortress { fortress_id } => {
            let fortress = game::fortress::food_collect(&client, &args.url, fortress_id).await?;
            println!("{fortress:?}");
        }
        Commands::GetWoodByFortress { fortress_id } => {
            let food = game::fortress::gold_get(&client, &args.url, fortress_id).await?;
            println!("{food:?}");
        }
        Commands::CollectWoodByFortress { fortress_id } => {
            let fortress = game::fortress::wood_collect(&client, &args.url, fortress_id).await?;
            println!("{fortress:?}");
        }
        Commands::GetEnergyByFortress { fortress_id } => {
            let energy = game::fortress::gold_get(&client, &args.url, fortress_id).await?;
            println!("{energy:?}");
        }
        Commands::CollectEnergyByFortress { fortress_id } => {
            let fortress = game::fortress::energy_collect(&client, &args.url, fortress_id).await?;
            println!("{fortress:?}");
        }
        Commands::GetAllBuildingsByFortress { fortress_id } => {
            let buildings =
                game::fortress::building_get_all(&client, &args.url, fortress_id).await?;
            println!("{buildings:?}");
        }
        Commands::GetAllBuildings => {
            let buildings = game::building::get_all(&client, &args.url).await?;
            println!("{buildings:?}");
        }
        Commands::GetBuilding { building_id } => {
            let building = game::building::get(&client, &args.url, building_id).await?;
            println!("{building:?}");
        }
        Commands::ImproveBuilding { building_id } => {
            let (fortress, building) =
                game::building::improve(&client, &args.url, building_id).await?;
            println!("{fortress:?} {building:?}");
        }
        Commands::GetImproveBuildingCosts { building_id } => {
            let costs = game::building::improve_costs(&client, &args.url, building_id).await?;
            println!("{costs:?}");
        }
    }
    Ok(())
}
