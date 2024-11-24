use reqwest::Client;

use crate::models::{Building, Fortress};

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn fortress_get_all(
    client: &Client,
    api_url: &str,
) -> Result<Vec<Fortress>, reqwest::Error> {
    let url = format!("{api_url}/api/fortress");
    let fortresses = client.get(url).send().await?.json().await?;
    Ok(fortresses)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn fortress_new(client: &Client, api_url: &str) -> Result<Fortress, reqwest::Error> {
    let url = format!("{api_url}/api/fortress/new");
    let fortress = client.get(url).send().await?.json().await?;
    Ok(fortress)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn fortress_get(
    client: &Client,
    api_url: &str,
    fortress_id: i32,
) -> Result<Fortress, reqwest::Error> {
    let url = format!("{api_url}/api/fortress/{fortress_id}");
    let fortress = client.get(url).send().await?.json().await?;
    Ok(fortress)
}

/// # Errors
///
/// Will return `Err` if the delete failed.
pub async fn fortress_delete(
    client: &Client,
    api_url: &str,
    fortress_id: i32,
) -> Result<usize, reqwest::Error> {
    let url = format!("{api_url}/api/fortress/{fortress_id}");
    let res = client.delete(url).send().await?.json().await?;
    Ok(res)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn fortress_gold_get(
    client: &Client,
    api_url: &str,
    fortress_id: i32,
) -> Result<i32, reqwest::Error> {
    let url = format!("{api_url}/api/fortress/{fortress_id}/gold");
    let gold = client.get(url).send().await?.json().await?;
    Ok(gold)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn fortress_gold_collect(
    client: &Client,
    api_url: &str,
    fortress_id: i32,
) -> Result<Fortress, reqwest::Error> {
    let url = format!("{api_url}/api/fortress/{fortress_id}/gold/collect");
    let fortress = client.get(url).send().await?.json().await?;
    Ok(fortress)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn fortress_food_get(
    client: &Client,
    api_url: &str,
    fortress_id: i32,
) -> Result<i32, reqwest::Error> {
    let url = format!("{api_url}/api/fortress/{fortress_id}/food");
    let food = client.get(url).send().await?.json().await?;
    Ok(food)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn fortress_food_collect(
    client: &Client,
    api_url: &str,
    fortress_id: i32,
) -> Result<Fortress, reqwest::Error> {
    let url = format!("{api_url}/api/fortress/{fortress_id}/food/collect");
    let fortress = client.get(url).send().await?.json().await?;
    Ok(fortress)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn fortress_wood_get(
    client: &Client,
    api_url: &str,
    fortress_id: i32,
) -> Result<i32, reqwest::Error> {
    let url = format!("{api_url}/api/fortress/{fortress_id}/wood");
    let wood = client.get(url).send().await?.json().await?;
    Ok(wood)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn fortress_wood_collect(
    client: &Client,
    api_url: &str,
    fortress_id: i32,
) -> Result<Fortress, reqwest::Error> {
    let url = format!("{api_url}/api/fortress/{fortress_id}/wood/collect");
    let fortress = client.get(url).send().await?.json().await?;
    Ok(fortress)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn fortress_energy_get(
    client: &Client,
    api_url: &str,
    fortress_id: i32,
) -> Result<i32, reqwest::Error> {
    let url = format!("{api_url}/api/fortress/{fortress_id}/energy");
    let energy = client.get(url).send().await?.json().await?;
    Ok(energy)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn fortress_energy_collect(
    client: &Client,
    api_url: &str,
    fortress_id: i32,
) -> Result<Fortress, reqwest::Error> {
    let url = format!("{api_url}/api/fortress/{fortress_id}/energy/collect");
    let fortress = client.get(url).send().await?.json().await?;
    Ok(fortress)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn fortress_building_get_all(
    client: &Client,
    api_url: &str,
    fortress_id: i32,
) -> Result<Vec<Building>, reqwest::Error> {
    let url = format!("{api_url}/api/fortress/{fortress_id}/building");
    let buildings = client.get(url).send().await?.json().await?;
    Ok(buildings)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn building_get_all(
    client: &Client,
    api_url: &str,
) -> Result<Vec<Building>, reqwest::Error> {
    let url = format!("{api_url}/api/building");
    let buildings = client.get(url).send().await?.json().await?;
    Ok(buildings)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn building_get(
    client: &Client,
    api_url: &str,
    building_id: i32,
) -> Result<Building, reqwest::Error> {
    let url = format!("{api_url}/api/building/{building_id}");
    let building = client.get(url).send().await?.json().await?;
    Ok(building)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn building_improve(
    client: &Client,
    api_url: &str,
    building_id: i32,
) -> Result<Building, reqwest::Error> {
    let url = format!("{api_url}/api/building/{building_id}/improve");
    let building = client.get(url).send().await?.json().await?;
    Ok(building)
}
