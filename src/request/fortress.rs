use super::API_URL;
use crate::models::{Fortress, NewFortress, UpdateFortress};
use reqwest::Client;

/// # Errors
///
/// Will return `Err` if the post failed.
pub async fn post(client: &Client, new_fortress: &NewFortress) -> Result<Fortress, reqwest::Error> {
    let url = format!("{API_URL}/fortress");
    let fortress = client
        .post(url)
        .json(&new_fortress)
        .send()
        .await?
        .json()
        .await?;
    Ok(fortress)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get_all(client: &Client) -> Result<Vec<Fortress>, reqwest::Error> {
    let url = format!("{API_URL}/fortress");
    let fortresses = client.get(url).send().await?.json().await?;
    Ok(fortresses)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get(client: &Client, id: i32) -> Result<Fortress, reqwest::Error> {
    let url = format!("{API_URL}/fortress/{id}");
    let fortress = client.get(url).send().await?.json().await?;
    Ok(fortress)
}

/// # Errors
///
/// Will return `Err` if the patch failed.
pub async fn patch(
    client: &Client,
    id: i32,
    update_fortress: &UpdateFortress,
) -> Result<Fortress, reqwest::Error> {
    let url = format!("{API_URL}/fortress/{id}");
    let fortress = client
        .patch(url)
        .json(&update_fortress)
        .send()
        .await?
        .json()
        .await?;
    Ok(fortress)
}

/// # Errors
///
/// Will return `Err` if the delete failed.
pub async fn delete(client: &Client, id: i32) -> Result<usize, reqwest::Error> {
    let url = format!("{API_URL}/fortress/{id}");
    let res = client.delete(url).send().await?.json().await?;
    Ok(res)
}
