use reqwest::Client;
use rusty_kingdom::{
    models::{NewBuilding, NewFortress, UpdateBuilding, UpdateFortress},
    request,
};

async fn test_fortress(client: &Client) -> Result<(), reqwest::Error> {
    println!("test fortress api:");
    {
        println!("test fortress post");
        let new_fortress = NewFortress {
            gold: 0,
            food: 0,
            wood: 0,
            energy: 0,
        };
        let fortress = request::fortress::post(client, &new_fortress).await?;
        println!("{fortress:#?}");
        println!("test fortress get");
        let fortress = request::fortress::get(client, fortress.id).await?;
        println!("{fortress:#?}");
        println!("test fortress patch");
        let update_fortress = UpdateFortress {
            gold: None,
            food: None,
            wood: None,
            energy: Some(42),
        };
        let fortress = request::fortress::patch(client, fortress.id, &update_fortress).await?;
        println!("{fortress:#?}");
        println!("test fortress delete");
        let res = request::fortress::delete(client, fortress.id).await?;
        println!("{res:#?}");
    }
    {
        let new_fortress = NewFortress {
            gold: 0,
            food: 0,
            wood: 0,
            energy: 0,
        };
        let fortress_0_id = request::fortress::post(client, &new_fortress).await?.id;
        let fortress_1_id = request::fortress::post(client, &new_fortress).await?.id;
        println!("test fortress get_all");
        let fortresses = request::fortress::get_all(client).await?;
        println!("{fortresses:#?}");
        let _ = request::fortress::delete(client, fortress_0_id).await?;
        let _ = request::fortress::delete(client, fortress_1_id).await?;
    }
    Ok(())
}

async fn test_building(client: &Client) -> Result<(), reqwest::Error> {
    println!("test building api:");
    let new_fortress = NewFortress {
        gold: 0,
        food: 0,
        wood: 0,
        energy: 0,
    };
    let fortress_id = request::fortress::post(client, &new_fortress).await?.id;
    {
        println!("test building post");
        let new_building = NewBuilding {
            name: "new building".to_string(),
            level: 0,
            fortress_id,
        };
        let building = request::building::post(client, &new_building).await?;
        println!("{building:#?}");
        println!("test building get");
        let building = request::building::get(client, building.id).await?;
        println!("{building:#?}");
        println!("test building patch");
        let update_building = UpdateBuilding {
            name: None,
            level: Some(42),
            fortress_id: None,
        };
        let building = request::building::patch(client, building.id, &update_building).await?;
        println!("{building:#?}");
        println!("test building delete");
        let res = request::building::delete(client, building.id).await?;
        println!("{res:#?}");
    }
    {
        let new_building = NewBuilding {
            name: "new building".to_string(),
            level: 0,
            fortress_id,
        };
        let _ = request::building::post(client, &new_building).await?;
        let _ = request::building::post(client, &new_building).await?;
        println!("test building get_by_fortress");
        let buildings = request::building::get_by_fortress(client, fortress_id).await?;
        println!("{buildings:#?}");
        println!("test building delete_by_fortress");
        let res = request::building::delete_by_fortress(client, fortress_id).await?;
        println!("{res:#?}");
    }
    {
        let new_building = NewBuilding {
            name: "new building".to_string(),
            level: 0,
            fortress_id,
        };
        let _ = request::building::post(client, &new_building).await?;
        let _ = request::building::post(client, &new_building).await?;
        println!("test building get_all");
        let buildings = request::building::get_all(client).await?;
        println!("{buildings:#?}");
    }
    let _ = request::fortress::delete(client, fortress_id).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    test_building(&client).await?;
    test_fortress(&client).await?;
    Ok(())
}
