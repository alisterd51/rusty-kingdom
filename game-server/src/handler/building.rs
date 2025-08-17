use super::internal_error;
use crate::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use rusty::{
    Costs, get_costs,
    models::{Building, Fortress},
    request::crud,
    upgrade_building,
};

const MAX_BUILDING_LEVEL: i32 = 20;

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get_all(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Building>>, (StatusCode, String)> {
    let buildings = crud::building::get_all(&app_state.client, &app_state.api_url)
        .await
        .map_err(internal_error)?;
    Ok(Json(buildings))
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get(
    Path(building_id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<Building>, (StatusCode, String)> {
    let building = crud::building::get(&app_state.client, &app_state.api_url, building_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(building))
}

/// # Errors
///
/// Will return `Err` if the get or patch failed.
pub async fn improve(
    Path(building_id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<(Fortress, Building)>, (StatusCode, String)> {
    let building = crud::building::get(&app_state.client, &app_state.api_url, building_id)
        .await
        .map_err(internal_error)?;
    let fortress = crud::fortress::get(&app_state.client, &app_state.api_url, building.fortress_id)
        .await
        .map_err(internal_error)?;
    let costs = get_costs(building.level, MAX_BUILDING_LEVEL);
    match upgrade_building(&fortress, &building, &costs, MAX_BUILDING_LEVEL) {
        Ok((update_fortress, update_building)) => {
            let fortress = crud::fortress::patch(
                &app_state.client,
                &app_state.api_url,
                fortress.id,
                &update_fortress,
            )
            .await
            .map_err(internal_error)?;
            let building = crud::building::patch(
                &app_state.client,
                &app_state.api_url,
                building.id,
                &update_building,
            )
            .await
            .map_err(internal_error)?;
            Ok(Json((fortress, building)))
        }
        Err(error) => Err((StatusCode::INTERNAL_SERVER_ERROR, error)),
    }
}

pub async fn improve_costs(
    Path(building_id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<Costs>, (StatusCode, String)> {
    let building = crud::building::get(&app_state.client, &app_state.api_url, building_id)
        .await
        .map_err(internal_error)?;
    let costs = get_costs(building.level, MAX_BUILDING_LEVEL);
    Ok(Json(costs))
}
