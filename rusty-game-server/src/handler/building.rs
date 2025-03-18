use super::internal_error;
use axum::{Json, extract::Path, http::StatusCode};
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
pub async fn get_all() -> Result<Json<Vec<Building>>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let buildings = crud::building::get_all(&client, &api_url)
        .await
        .map_err(internal_error)?;
    Ok(Json(buildings))
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get(Path(building_id): Path<i32>) -> Result<Json<Building>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let building = crud::building::get(&client, &api_url, building_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(building))
}

/// # Errors
///
/// Will return `Err` if the get or patch failed.
pub async fn improve(
    Path(building_id): Path<i32>,
) -> Result<Json<(Fortress, Building)>, (StatusCode, String)> {
    // TODO: `api_url` can be generated only once in main and passed as a parameter to this function (performance to be tested at extremely high throughput without DB interaction)
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let building = crud::building::get(&client, &api_url, building_id)
        .await
        .map_err(internal_error)?;
    let fortress = crud::fortress::get(&client, &api_url, building.fortress_id)
        .await
        .map_err(internal_error)?;
    let costs = get_costs(building.level, MAX_BUILDING_LEVEL);
    match upgrade_building(&fortress, &building, &costs, MAX_BUILDING_LEVEL) {
        Ok((update_fortress, update_building)) => {
            let fortress = crud::fortress::patch(&client, &api_url, fortress.id, &update_fortress)
                .await
                .map_err(internal_error)?;
            let building = crud::building::patch(&client, &api_url, building.id, &update_building)
                .await
                .map_err(internal_error)?;
            Ok(Json((fortress, building)))
        }
        Err(error) => Err((StatusCode::INTERNAL_SERVER_ERROR, error)),
    }
}

pub async fn improve_costs(
    Path(building_id): Path<i32>,
) -> Result<Json<Costs>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let building = crud::building::get(&client, &api_url, building_id)
        .await
        .map_err(internal_error)?;
    let costs = get_costs(building.level, MAX_BUILDING_LEVEL);
    Ok(Json(costs))
}
