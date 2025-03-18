use super::internal_error;
use axum::{Json, extract::Path};
use reqwest::StatusCode;
use rusty::{
    get_energy_bonus, get_food_bonus, get_gold_bonus, get_wood_bonus,
    models::{Building, Fortress, NewBuilding, NewFortress, UpdateFortress},
    request::crud,
};

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get_all() -> Result<Json<Vec<Fortress>>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let fortresses = crud::fortress::get_all(&client, &api_url)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortresses))
}

/// # Errors
///
/// Will return `Err` if the post failed.
pub async fn new() -> Result<Json<(Fortress, Vec<Building>)>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let new_fortress = NewFortress::new();
    let fortress = crud::fortress::post(&client, &api_url, &new_fortress)
        .await
        .map_err(internal_error)?;
    let new_buildings = vec![
        NewBuilding::new("bank".to_string(), fortress.id),
        NewBuilding::new("farm".to_string(), fortress.id),
        NewBuilding::new("sawmill".to_string(), fortress.id),
        NewBuilding::new("sanctuary".to_string(), fortress.id),
    ];
    let mut buildings = vec![];
    for new_building in new_buildings {
        let building = crud::building::post(&client, &api_url, &new_building)
            .await
            .map_err(internal_error)?;
        buildings.push(building);
    }
    Ok(Json((fortress, buildings)))
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get(Path(fortress_id): Path<i32>) -> Result<Json<Fortress>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let fortress = crud::fortress::get(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortress))
}

/// # Errors
///
/// Will return `Err` if the delete failed.
pub async fn delete(Path(fortress_id): Path<i32>) -> Result<Json<usize>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let res = crud::fortress::delete(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(res))
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn gold_get(Path(fortress_id): Path<i32>) -> Result<Json<i32>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let fortress = crud::fortress::get(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortress.gold))
}

/// # Errors
///
/// Will return `Err` if the get or patch failed.
pub async fn gold_collect(
    Path(fortress_id): Path<i32>,
) -> Result<Json<Fortress>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let fortress = crud::fortress::get(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    let buildings = crud::building::get_by_fortress(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    let gold_bonus = get_gold_bonus(buildings);
    let update_fortress = UpdateFortress {
        gold: Some(fortress.gold + 1 + gold_bonus),
        food: None,
        wood: None,
        energy: None,
    };
    let fortress = crud::fortress::patch(&client, &api_url, fortress_id, &update_fortress)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortress))
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn food_get(Path(fortress_id): Path<i32>) -> Result<Json<i32>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let fortress = crud::fortress::get(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortress.food))
}

/// # Errors
///
/// Will return `Err` if the get or patch failed.
pub async fn food_collect(
    Path(fortress_id): Path<i32>,
) -> Result<Json<Fortress>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let fortress = crud::fortress::get(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    let buildings = crud::building::get_by_fortress(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    let food_bonus = get_food_bonus(buildings);
    let update_fortress = UpdateFortress {
        gold: None,
        food: Some(fortress.food + 1 + food_bonus),
        wood: None,
        energy: None,
    };
    let fortress = crud::fortress::patch(&client, &api_url, fortress_id, &update_fortress)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortress))
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn wood_get(Path(fortress_id): Path<i32>) -> Result<Json<i32>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let fortress = crud::fortress::get(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortress.wood))
}

/// # Errors
///
/// Will return `Err` if the get or patch failed.
pub async fn wood_collect(
    Path(fortress_id): Path<i32>,
) -> Result<Json<Fortress>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let fortress = crud::fortress::get(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    let buildings = crud::building::get_by_fortress(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    let wood_bonus = get_wood_bonus(buildings);
    let update_fortress = UpdateFortress {
        gold: None,
        food: None,
        wood: Some(fortress.wood + 1 + wood_bonus),
        energy: None,
    };
    let fortress = crud::fortress::patch(&client, &api_url, fortress_id, &update_fortress)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortress))
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn energy_get(Path(fortress_id): Path<i32>) -> Result<Json<i32>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let fortress = crud::fortress::get(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortress.energy))
}

/// # Errors
///
/// Will return `Err` if the get or patch failed.
pub async fn energy_collect(
    Path(fortress_id): Path<i32>,
) -> Result<Json<Fortress>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let fortress = crud::fortress::get(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    let buildings = crud::building::get_by_fortress(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    let energy_bonus = get_energy_bonus(buildings);
    let update_fortress = UpdateFortress {
        gold: None,
        food: None,
        wood: None,
        energy: Some(fortress.energy + 1 + energy_bonus),
    };
    let fortress = crud::fortress::patch(&client, &api_url, fortress_id, &update_fortress)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortress))
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn building_get_all(
    Path(fortress_id): Path<i32>,
) -> Result<Json<Vec<Building>>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let buildings = crud::building::get_by_fortress(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(buildings))
}
