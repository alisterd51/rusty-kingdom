use crate::models::{Fortress, NewFortress, UpdateFortress};
use reqwest::Client;

/// # Errors
///
/// Will return `Err` if the post failed.
pub async fn post(
    client: &Client,
    api_url: &str,
    new_fortress: &NewFortress,
) -> Result<Fortress, reqwest::Error> {
    let url = format!("{api_url}/api/fortress");
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
pub async fn get_all(client: &Client, api_url: &str) -> Result<Vec<Fortress>, reqwest::Error> {
    let url = format!("{api_url}/api/fortress");
    let fortresses = client.get(url).send().await?.json().await?;
    Ok(fortresses)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get(client: &Client, api_url: &str, id: i32) -> Result<Fortress, reqwest::Error> {
    let url = format!("{api_url}/api/fortress/{id}");
    let fortress = client.get(url).send().await?.json().await?;
    Ok(fortress)
}

/// # Errors
///
/// Will return `Err` if the patch failed.
pub async fn patch(
    client: &Client,
    api_url: &str,
    id: i32,
    update_fortress: &UpdateFortress,
) -> Result<Fortress, reqwest::Error> {
    let url = format!("{api_url}/api/fortress/{id}");
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
pub async fn delete(client: &Client, api_url: &str, id: i32) -> Result<usize, reqwest::Error> {
    let url = format!("{api_url}/api/fortress/{id}");
    let res = client.delete(url).send().await?.json().await?;
    Ok(res)
}
