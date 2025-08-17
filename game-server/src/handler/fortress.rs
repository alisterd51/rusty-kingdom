use super::internal_error;
use crate::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use reqwest::StatusCode;
use rusty::{
    get_energy_bonus, get_food_bonus, get_gold_bonus, get_wood_bonus,
    models::{Building, Fortress, NewBuilding, NewFortress, UpdateFortress},
    request::crud,
};

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get_all(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Fortress>>, (StatusCode, String)> {
    let fortresses = crud::fortress::get_all(&app_state.client, &app_state.api_url)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortresses))
}

/// # Errors
///
/// Will return `Err` if the post failed.
pub async fn new(
    State(app_state): State<AppState>,
) -> Result<Json<(Fortress, Vec<Building>)>, (StatusCode, String)> {
    let new_fortress = NewFortress::new();
    let fortress = crud::fortress::post(&app_state.client, &app_state.api_url, &new_fortress)
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
        let building = crud::building::post(&app_state.client, &app_state.api_url, &new_building)
            .await
            .map_err(internal_error)?;
        buildings.push(building);
    }
    Ok(Json((fortress, buildings)))
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get(
    Path(fortress_id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<Fortress>, (StatusCode, String)> {
    let fortress = crud::fortress::get(&app_state.client, &app_state.api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortress))
}

/// # Errors
///
/// Will return `Err` if the delete failed.
pub async fn delete(
    Path(fortress_id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<usize>, (StatusCode, String)> {
    let res = crud::fortress::delete(&app_state.client, &app_state.api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(res))
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn gold_get(
    Path(fortress_id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<i32>, (StatusCode, String)> {
    let fortress = crud::fortress::get(&app_state.client, &app_state.api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortress.gold))
}

/// # Errors
///
/// Will return `Err` if the get or patch failed.
pub async fn gold_collect(
    Path(fortress_id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<Fortress>, (StatusCode, String)> {
    let fortress = crud::fortress::get(&app_state.client, &app_state.api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    let buildings =
        crud::building::get_by_fortress(&app_state.client, &app_state.api_url, fortress_id)
            .await
            .map_err(internal_error)?;
    let gold_bonus = get_gold_bonus(buildings);
    let update_fortress = UpdateFortress {
        gold: Some(fortress.gold + 1 + gold_bonus),
        food: None,
        wood: None,
        energy: None,
    };
    let fortress = crud::fortress::patch(
        &app_state.client,
        &app_state.api_url,
        fortress_id,
        &update_fortress,
    )
    .await
    .map_err(internal_error)?;
    Ok(Json(fortress))
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn food_get(
    Path(fortress_id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<i32>, (StatusCode, String)> {
    let fortress = crud::fortress::get(&app_state.client, &app_state.api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortress.food))
}

/// # Errors
///
/// Will return `Err` if the get or patch failed.
pub async fn food_collect(
    Path(fortress_id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<Fortress>, (StatusCode, String)> {
    let fortress = crud::fortress::get(&app_state.client, &app_state.api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    let buildings =
        crud::building::get_by_fortress(&app_state.client, &app_state.api_url, fortress_id)
            .await
            .map_err(internal_error)?;
    let food_bonus = get_food_bonus(buildings);
    let update_fortress = UpdateFortress {
        gold: None,
        food: Some(fortress.food + 1 + food_bonus),
        wood: None,
        energy: None,
    };
    let fortress = crud::fortress::patch(
        &app_state.client,
        &app_state.api_url,
        fortress_id,
        &update_fortress,
    )
    .await
    .map_err(internal_error)?;
    Ok(Json(fortress))
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn wood_get(
    Path(fortress_id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<i32>, (StatusCode, String)> {
    let fortress = crud::fortress::get(&app_state.client, &app_state.api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortress.wood))
}

/// # Errors
///
/// Will return `Err` if the get or patch failed.
pub async fn wood_collect(
    Path(fortress_id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<Fortress>, (StatusCode, String)> {
    let fortress = crud::fortress::get(&app_state.client, &app_state.api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    let buildings =
        crud::building::get_by_fortress(&app_state.client, &app_state.api_url, fortress_id)
            .await
            .map_err(internal_error)?;
    let wood_bonus = get_wood_bonus(buildings);
    let update_fortress = UpdateFortress {
        gold: None,
        food: None,
        wood: Some(fortress.wood + 1 + wood_bonus),
        energy: None,
    };
    let fortress = crud::fortress::patch(
        &app_state.client,
        &app_state.api_url,
        fortress_id,
        &update_fortress,
    )
    .await
    .map_err(internal_error)?;
    Ok(Json(fortress))
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn energy_get(
    Path(fortress_id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<i32>, (StatusCode, String)> {
    let fortress = crud::fortress::get(&app_state.client, &app_state.api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortress.energy))
}

/// # Errors
///
/// Will return `Err` if the get or patch failed.
pub async fn energy_collect(
    Path(fortress_id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<Fortress>, (StatusCode, String)> {
    let fortress = crud::fortress::get(&app_state.client, &app_state.api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    let buildings =
        crud::building::get_by_fortress(&app_state.client, &app_state.api_url, fortress_id)
            .await
            .map_err(internal_error)?;
    let energy_bonus = get_energy_bonus(buildings);
    let update_fortress = UpdateFortress {
        gold: None,
        food: None,
        wood: None,
        energy: Some(fortress.energy + 1 + energy_bonus),
    };
    let fortress = crud::fortress::patch(
        &app_state.client,
        &app_state.api_url,
        fortress_id,
        &update_fortress,
    )
    .await
    .map_err(internal_error)?;
    Ok(Json(fortress))
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn building_get_all(
    Path(fortress_id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Building>>, (StatusCode, String)> {
    let buildings =
        crud::building::get_by_fortress(&app_state.client, &app_state.api_url, fortress_id)
            .await
            .map_err(internal_error)?;
    Ok(Json(buildings))
}
