use super::API_URL;
use crate::models::{Building, NewBuilding, UpdateBuilding};
use reqwest::Client;

/// # Errors
///
/// Will return `Err` if the post failed.
pub async fn post(client: &Client, new_building: &NewBuilding) -> Result<Building, reqwest::Error> {
    let url = format!("{API_URL}/building");
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
pub async fn get_all(client: &Client) -> Result<Vec<Building>, reqwest::Error> {
    let url = format!("{API_URL}/building");
    let buildings = client.get(url).send().await?.json().await?;
    Ok(buildings)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get(client: &Client, id: i32) -> Result<Building, reqwest::Error> {
    let url = format!("{API_URL}/building/{id}");
    let building = client.get(url).send().await?.json().await?;
    Ok(building)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get_by_fortress(
    client: &Client,
    fortress_id: i32,
) -> Result<Vec<Building>, reqwest::Error> {
    let url = format!("{API_URL}/fortress/{fortress_id}/building");
    let buildings = client.get(url).send().await?.json().await?;
    Ok(buildings)
}

/// # Errors
///
/// Will return `Err` if the patch failed.
pub async fn patch(
    client: &Client,
    id: i32,
    update_building: &UpdateBuilding,
) -> Result<Building, reqwest::Error> {
    let url = format!("{API_URL}/building/{id}");
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
pub async fn delete(client: &Client, id: i32) -> Result<usize, reqwest::Error> {
    let url = format!("{API_URL}/building/{id}");
    let res = client.delete(url).send().await?.json().await?;
    Ok(res)
}

/// # Errors
///
/// Will return `Err` if the delete failed.
pub async fn delete_by_fortress(
    client: &Client,
    fortress_id: i32,
) -> Result<usize, reqwest::Error> {
    let url = format!("{API_URL}/fortress/{fortress_id}/building");
    let res = client.delete(url).send().await?.json().await?;
    Ok(res)
}
