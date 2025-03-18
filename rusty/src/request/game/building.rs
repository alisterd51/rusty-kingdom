use crate::{
    Costs,
    models::{Building, Fortress},
};
use reqwest::Client;

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get_all(client: &Client, api_url: &str) -> Result<Vec<Building>, String> {
    let url = format!("{api_url}/api/building");
    let resp = client.get(url).send().await.map_err(|e| e.to_string())?;
    let buildings = resp.json().await.map_err(|e| e.to_string())?;
    Ok(buildings)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get(client: &Client, api_url: &str, building_id: i32) -> Result<Building, String> {
    let url = format!("{api_url}/api/building/{building_id}");
    let resp = client.get(url).send().await.map_err(|e| e.to_string())?;
    let building = resp.json().await.map_err(|e| e.to_string())?;
    Ok(building)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn improve(
    client: &Client,
    api_url: &str,
    building_id: i32,
) -> Result<(Fortress, Building), String> {
    let url = format!("{api_url}/api/building/{building_id}/improve");
    let resp = client.get(url).send().await.map_err(|e| e.to_string())?;
    if resp.status() == reqwest::StatusCode::OK {
        let (fortress, building) = resp.json().await.map_err(|e| e.to_string())?;
        Ok((fortress, building))
    } else {
        let error = resp.bytes().await.map_err(|e| e.to_string())?;
        let error = String::from_utf8(error.to_vec()).map_err(|e| e.to_string())?;
        Err(error)
    }
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn improve_costs(
    client: &Client,
    api_url: &str,
    building_id: i32,
) -> Result<Costs, String> {
    let url = format!("{api_url}/api/building/{building_id}/improve/costs");
    let resp = client.get(url).send().await.map_err(|e| e.to_string())?;
    let costs = resp.json().await.map_err(|e| e.to_string())?;
    Ok(costs)
}
