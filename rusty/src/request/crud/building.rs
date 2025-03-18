use crate::models::{Building, NewBuilding, UpdateBuilding};
use reqwest::Client;

/// # Errors
///
/// Will return `Err` if the post failed.
pub async fn post(
    client: &Client,
    api_url: &str,
    new_building: &NewBuilding,
) -> Result<Building, reqwest::Error> {
    let url = format!("{api_url}/api/building");
    let building = client
        .post(url)
        .json(&new_building)
        .send()
        .await?
        .json()
        .await?;
    Ok(building)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get_all(client: &Client, api_url: &str) -> Result<Vec<Building>, reqwest::Error> {
    let url = format!("{api_url}/api/building");
    let buildings = client.get(url).send().await?.json().await?;
    Ok(buildings)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get(client: &Client, api_url: &str, id: i32) -> Result<Building, reqwest::Error> {
    let url = format!("{api_url}/api/building/{id}");
    let building = client.get(url).send().await?.json().await?;
    Ok(building)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get_by_fortress(
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
/// Will return `Err` if the patch failed.
pub async fn patch(
    client: &Client,
    api_url: &str,
    id: i32,
    update_building: &UpdateBuilding,
) -> Result<Building, reqwest::Error> {
    let url = format!("{api_url}/api/building/{id}");
    let building = client
        .patch(url)
        .json(&update_building)
        .send()
        .await?
        .json()
        .await?;
    Ok(building)
}

/// # Errors
///
/// Will return `Err` if the delete failed.
pub async fn delete(client: &Client, api_url: &str, id: i32) -> Result<usize, reqwest::Error> {
    let url = format!("{api_url}/api/building/{id}");
    let res = client.delete(url).send().await?.json().await?;
    Ok(res)
}

/// # Errors
///
/// Will return `Err` if the delete failed.
pub async fn delete_by_fortress(
    client: &Client,
    api_url: &str,
    fortress_id: i32,
) -> Result<usize, reqwest::Error> {
    let url = format!("{api_url}/api/fortress/{fortress_id}/building");
    let res = client.delete(url).send().await?.json().await?;
    Ok(res)
}
