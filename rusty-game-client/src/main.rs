use clap::{Command, CommandFactory, Parser, Subcommand};
use clap_complete::{Generator, Shell, generate};
use rusty::request::game;
use serde_json::json;
use std::io;

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

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Args::parse();
    let client = reqwest::Client::new();

    match args.cmd {
        Commands::Fortress { cmd } => match cmd {
            FortressCommands::GetAll => {
                let fortresses = game::fortress::get_all(&client, &args.url).await?;
                println!("{}", json!(fortresses));
            }
            FortressCommands::New => {
                let (fortress, buildings) = game::fortress::new(&client, &args.url).await?;
                println!("{}", json!((fortress, buildings)));
            }
            FortressCommands::Get { fortress_id } => {
                let fortress = game::fortress::get(&client, &args.url, fortress_id).await?;
                println!("{}", json!(fortress));
            }
            FortressCommands::Delete { fortress_id } => {
                let deleted = game::fortress::delete(&client, &args.url, fortress_id).await?;
                println!("{}", json!(deleted));
            }
            FortressCommands::GetGold { fortress_id } => {
                let gold = game::fortress::gold_get(&client, &args.url, fortress_id).await?;
                println!("{}", json!(gold));
            }
            FortressCommands::CollectGold { fortress_id } => {
                let fortress =
                    game::fortress::gold_collect(&client, &args.url, fortress_id).await?;
                println!("{}", json!(fortress));
            }
            FortressCommands::GetFood { fortress_id } => {
                let food = game::fortress::food_get(&client, &args.url, fortress_id).await?;
                println!("{}", json!(food));
            }
            FortressCommands::CollectFood { fortress_id } => {
                let fortress =
                    game::fortress::food_collect(&client, &args.url, fortress_id).await?;
                println!("{}", json!(fortress));
            }
            FortressCommands::GetWood { fortress_id } => {
                let wood = game::fortress::wood_get(&client, &args.url, fortress_id).await?;
                println!("{}", json!(wood));
            }
            FortressCommands::CollectWood { fortress_id } => {
                let fortress =
                    game::fortress::wood_collect(&client, &args.url, fortress_id).await?;
                println!("{}", json!(fortress));
            }
            FortressCommands::GetEnergy { fortress_id } => {
                let energy = game::fortress::energy_get(&client, &args.url, fortress_id).await?;
                println!("{}", json!(energy));
            }
            FortressCommands::CollectEnergy { fortress_id } => {
                let fortress =
                    game::fortress::energy_collect(&client, &args.url, fortress_id).await?;
                println!("{}", json!(fortress));
            }
            FortressCommands::GetAllBuildings { fortress_id } => {
                let buildings =
                    game::fortress::building_get_all(&client, &args.url, fortress_id).await?;
                println!("{}", json!(buildings));
            }
        },
        Commands::Building { cmd } => match cmd {
            BuildingCommands::GetAll => {
                let buildings = game::building::get_all(&client, &args.url).await?;
                println!("{}", json!(buildings));
            }
            BuildingCommands::Get { building_id } => {
                let building = game::building::get(&client, &args.url, building_id).await?;
                println!("{}", json!(building));
            }
            BuildingCommands::Improve { building_id } => {
                let (fortress, building) =
                    game::building::improve(&client, &args.url, building_id).await?;
                println!("{}", json!((fortress, building)));
            }
            BuildingCommands::GetImproveCosts { building_id } => {
                let costs = game::building::improve_costs(&client, &args.url, building_id).await?;
                println!("{}", json!(costs));
            }
        },
        Commands::Bench { size } => {
            let (fortress, _buildings) = game::fortress::new(&client, &args.url).await?;
            for _ in 0..size {
                let _ = game::fortress::gold_collect(&client, &args.url, fortress.id).await;
            }
            let gold = game::fortress::gold_get(&client, &args.url, fortress.id).await?;
            println!("gold: {gold}");
            game::fortress::delete(&client, &args.url, fortress.id).await?;
        }
        Commands::Completions { shell } => {
            let mut cmd = Args::command();
            print_completions(shell, &mut cmd);
        }
    }
    Ok(())
}
