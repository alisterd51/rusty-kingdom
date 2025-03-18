use reqwest::Client;
use rusty::{
    models::{NewBuilding, NewFortress, UpdateBuilding, UpdateFortress},
    request::crud,
};

pub const API_URL: &str = "http://localhost:3000";

async fn test_fortress(client: &Client) -> Result<(), reqwest::Error> {
    println!("test fortress api:");
    {
        println!("test fortress post");
        let new_fortress = NewFortress::new();
        let fortress = crud::fortress::post(client, API_URL, &new_fortress).await?;
        println!("{fortress:#?}");
        println!("test fortress get");
        let fortress = crud::fortress::get(client, API_URL, fortress.id).await?;
        println!("{fortress:#?}");
        println!("test fortress patch");
        let update_fortress = UpdateFortress {
            gold: None,
            food: None,
            wood: None,
            energy: Some(42),
        };
        let fortress =
            crud::fortress::patch(client, API_URL, fortress.id, &update_fortress).await?;
        println!("{fortress:#?}");
        println!("test fortress delete");
        let res = crud::fortress::delete(client, API_URL, fortress.id).await?;
        println!("{res:#?}");
    }
    {
        let new_fortress = NewFortress::new();
        let fortress_0_id = crud::fortress::post(client, API_URL, &new_fortress)
            .await?
            .id;
        let fortress_1_id = crud::fortress::post(client, API_URL, &new_fortress)
            .await?
            .id;
        println!("test fortress get_all");
        let fortresses = crud::fortress::get_all(client, API_URL).await?;
        println!("{fortresses:#?}");
        let _ = crud::fortress::delete(client, API_URL, fortress_0_id).await?;
        let _ = crud::fortress::delete(client, API_URL, fortress_1_id).await?;
    }
    Ok(())
}

async fn test_building(client: &Client) -> Result<(), reqwest::Error> {
    println!("test building api:");
    let new_fortress = NewFortress::new();
    let fortress_id = crud::fortress::post(client, API_URL, &new_fortress)
        .await?
        .id;
    {
        println!("test building post");
        let new_building = NewBuilding::new("new building".to_string(), fortress_id);
        let building = crud::building::post(client, API_URL, &new_building).await?;
        println!("{building:#?}");
        println!("test building get");
        let building = crud::building::get(client, API_URL, building.id).await?;
        println!("{building:#?}");
        println!("test building patch");
        let update_building = UpdateBuilding {
            name: None,
            level: Some(42),
            fortress_id: None,
        };
        let building =
            crud::building::patch(client, API_URL, building.id, &update_building).await?;
        println!("{building:#?}");
        println!("test building delete");
        let res = crud::building::delete(client, API_URL, building.id).await?;
        println!("{res:#?}");
    }
    {
        let new_building = NewBuilding::new("new building".to_string(), fortress_id);
        let _ = crud::building::post(client, API_URL, &new_building).await?;
        let _ = crud::building::post(client, API_URL, &new_building).await?;
        println!("test building get_by_fortress");
        let buildings = crud::building::get_by_fortress(client, API_URL, fortress_id).await?;
        println!("{buildings:#?}");
        println!("test building delete_by_fortress");
        let res = crud::building::delete_by_fortress(client, API_URL, fortress_id).await?;
        println!("{res:#?}");
    }
    {
        let new_building = NewBuilding::new("new building".to_string(), fortress_id);
        let _ = crud::building::post(client, API_URL, &new_building).await?;
        let _ = crud::building::post(client, API_URL, &new_building).await?;
        println!("test building get_all");
        let buildings = crud::building::get_all(client, API_URL).await?;
        println!("{buildings:#?}");
    }
    let _ = crud::fortress::delete(client, API_URL, fortress_id).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    test_building(&client).await?;
    test_fortress(&client).await?;
    Ok(())
}
