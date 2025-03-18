use crate::models::{Building, Fortress};
use reqwest::Client;

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get_all(client: &Client, api_url: &str) -> Result<Vec<Fortress>, String> {
    let url = format!("{api_url}/api/fortress");
    let resp = client.get(url).send().await.map_err(|e| e.to_string())?;
    let fortresses = resp.json().await.map_err(|e| e.to_string())?;
    Ok(fortresses)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn new(client: &Client, api_url: &str) -> Result<(Fortress, Vec<Building>), String> {
    let url = format!("{api_url}/api/fortress/new");
    let resp = client.get(url).send().await.map_err(|e| e.to_string())?;
    let fortress = resp.json().await.map_err(|e| e.to_string())?;
    Ok(fortress)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get(client: &Client, api_url: &str, fortress_id: i32) -> Result<Fortress, String> {
    let url = format!("{api_url}/api/fortress/{fortress_id}");
    let resp = client.get(url).send().await.map_err(|e| e.to_string())?;
    let fortress = resp.json().await.map_err(|e| e.to_string())?;
    Ok(fortress)
}

/// # Errors
///
/// Will return `Err` if the delete failed.
pub async fn delete(client: &Client, api_url: &str, fortress_id: i32) -> Result<usize, String> {
    let url = format!("{api_url}/api/fortress/{fortress_id}");
    let resp = client.delete(url).send().await.map_err(|e| e.to_string())?;
    let res = resp.json().await.map_err(|e| e.to_string())?;
    Ok(res)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn gold_get(client: &Client, api_url: &str, fortress_id: i32) -> Result<i32, String> {
    let url = format!("{api_url}/api/fortress/{fortress_id}/gold");
    let resp = client.get(url).send().await.map_err(|e| e.to_string())?;
    let gold = resp.json().await.map_err(|e| e.to_string())?;
    Ok(gold)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn gold_collect(
    client: &Client,
    api_url: &str,
    fortress_id: i32,
) -> Result<Fortress, String> {
    let url = format!("{api_url}/api/fortress/{fortress_id}/gold/collect");
    let resp = client.get(url).send().await.map_err(|e| e.to_string())?;
    let fortress = resp.json().await.map_err(|e| e.to_string())?;
    Ok(fortress)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn food_get(client: &Client, api_url: &str, fortress_id: i32) -> Result<i32, String> {
    let url = format!("{api_url}/api/fortress/{fortress_id}/food");
    let resp = client.get(url).send().await.map_err(|e| e.to_string())?;
    let food = resp.json().await.map_err(|e| e.to_string())?;
    Ok(food)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn food_collect(
    client: &Client,
    api_url: &str,
    fortress_id: i32,
) -> Result<Fortress, String> {
    let url = format!("{api_url}/api/fortress/{fortress_id}/food/collect");
    let resp = client.get(url).send().await.map_err(|e| e.to_string())?;
    let fortress = resp.json().await.map_err(|e| e.to_string())?;
    Ok(fortress)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn wood_get(client: &Client, api_url: &str, fortress_id: i32) -> Result<i32, String> {
    let url = format!("{api_url}/api/fortress/{fortress_id}/wood");
    let resp = client.get(url).send().await.map_err(|e| e.to_string())?;
    let wood = resp.json().await.map_err(|e| e.to_string())?;
    Ok(wood)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn wood_collect(
    client: &Client,
    api_url: &str,
    fortress_id: i32,
) -> Result<Fortress, String> {
    let url = format!("{api_url}/api/fortress/{fortress_id}/wood/collect");
    let resp = client.get(url).send().await.map_err(|e| e.to_string())?;
    let fortress = resp.json().await.map_err(|e| e.to_string())?;
    Ok(fortress)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn energy_get(client: &Client, api_url: &str, fortress_id: i32) -> Result<i32, String> {
    let url = format!("{api_url}/api/fortress/{fortress_id}/energy");
    let resp = client.get(url).send().await.map_err(|e| e.to_string())?;
    let energy = resp.json().await.map_err(|e| e.to_string())?;
    Ok(energy)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn energy_collect(
    client: &Client,
    api_url: &str,
    fortress_id: i32,
) -> Result<Fortress, String> {
    let url = format!("{api_url}/api/fortress/{fortress_id}/energy/collect");
    let resp = client.get(url).send().await.map_err(|e| e.to_string())?;
    let fortress = resp.json().await.map_err(|e| e.to_string())?;
    Ok(fortress)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn building_get_all(
    client: &Client,
    api_url: &str,
    fortress_id: i32,
) -> Result<Vec<Building>, String> {
    let url = format!("{api_url}/api/fortress/{fortress_id}/building");
    let resp = client.get(url).send().await.map_err(|e| e.to_string())?;
    let buildings = resp.json().await.map_err(|e| e.to_string())?;
    Ok(buildings)
}
