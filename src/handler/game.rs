use super::internal_error;
use crate::{
    models::{Building, Fortress, NewFortress, UpdateBuilding, UpdateFortress},
    request,
};
use axum::{extract::Path, Json};
use reqwest::StatusCode;

const FORTRESS_TAG: &str = "fortress";
const BUILDING_TAG: &str = "building";

/// # Errors
///
/// Will return `Err` if the get failed.
#[utoipa::path(
    get,
    path = "",
    tag = FORTRESS_TAG,
    responses(
        (status = 200, description = "List all fortress", body = [Fortress])
    )
)]
pub async fn fortress_get_all() -> Result<Json<Vec<Fortress>>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let fortresses = request::fortress::get_all(&client, &api_url)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortresses))
}

/// # Errors
///
/// Will return `Err` if the post failed.
#[utoipa::path(
    get,
    path = "/new",
    tag = FORTRESS_TAG,
    responses(
        (status = 200, description = "Create new fortress", body = Fortress)
    )
)]
pub async fn fortress_new() -> Result<Json<Fortress>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let new_fortress = NewFortress {
        gold: 0,
        food: 0,
        wood: 0,
        energy: 0,
    };
    let fortress = request::fortress::post(&client, &api_url, &new_fortress)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortress))
}

/// # Errors
///
/// Will return `Err` if the get failed.
#[utoipa::path(
    get,
    path = "/{fortress_id}",
    tag = FORTRESS_TAG,
    responses(
        (status = 200, description = "Get one fortress", body = Fortress)
    )
)]
pub async fn fortress_get(
    Path(fortress_id): Path<i32>,
) -> Result<Json<Fortress>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let fortress = request::fortress::get(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortress))
}

/// # Errors
///
/// Will return `Err` if the delete failed.
#[utoipa::path(
    delete,
    path = "/{fortress_id}",
    tag = FORTRESS_TAG,
    responses(
        (status = 200, description = "Delete one fortress", body = usize)
    )
)]
pub async fn fortress_delete(
    Path(fortress_id): Path<i32>,
) -> Result<Json<usize>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let res = request::fortress::delete(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(res))
}

/// # Errors
///
/// Will return `Err` if the get failed.
#[utoipa::path(
    get,
    path = "/{fortress_id}/gold",
    tag = FORTRESS_TAG,
    responses(
        (status = 200, description = "get gold of fortress", body = i32)
    )
)]
pub async fn fortress_gold_get(
    Path(fortress_id): Path<i32>,
) -> Result<Json<i32>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let fortress = request::fortress::get(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortress.gold))
}

/// # Errors
///
/// Will return `Err` if the get or patch failed.
#[utoipa::path(
    get,
    path = "/{fortress_id}/gold/collect",
    tag = FORTRESS_TAG,
    responses(
        (status = 200, description = "Collect more gold", body = Fortress)
    )
)]
pub async fn fortress_gold_collect(
    Path(fortress_id): Path<i32>,
) -> Result<Json<Fortress>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let fortress = request::fortress::get(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    // TODO calculer le nouveau montant de gold avec un truc bien
    let update_fortress = UpdateFortress {
        gold: Some(fortress.gold + 1),
        food: None,
        wood: None,
        energy: None,
    };
    let fortress = request::fortress::patch(&client, &api_url, fortress_id, &update_fortress)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortress))
}

/// # Errors
///
/// Will return `Err` if the get failed.
#[utoipa::path(
    get,
    path = "/{fortress_id}/food",
    tag = FORTRESS_TAG,
    responses(
        (status = 200, description = "get food of fortress", body = i32)
    )
)]
pub async fn fortress_food_get(
    Path(fortress_id): Path<i32>,
) -> Result<Json<i32>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let fortress = request::fortress::get(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortress.food))
}

/// # Errors
///
/// Will return `Err` if the get or patch failed.
#[utoipa::path(
    get,
    path = "/{fortress_id}/food/collect",
    tag = FORTRESS_TAG,
    responses(
        (status = 200, description = "Collect more food", body = Fortress)
    )
)]
pub async fn fortress_food_collect(
    Path(fortress_id): Path<i32>,
) -> Result<Json<Fortress>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let fortress = request::fortress::get(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    // TODO calculer le nouveau montant de gold avec un truc bien
    let update_fortress = UpdateFortress {
        gold: None,
        food: Some(fortress.food + 1),
        wood: None,
        energy: None,
    };
    let fortress = request::fortress::patch(&client, &api_url, fortress_id, &update_fortress)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortress))
}

/// # Errors
///
/// Will return `Err` if the get failed.
#[utoipa::path(
    get,
    path = "/{fortress_id}/wood",
    tag = FORTRESS_TAG,
    responses(
        (status = 200, description = "get wood of fortress", body = i32)
    )
)]
pub async fn fortress_wood_get(
    Path(fortress_id): Path<i32>,
) -> Result<Json<i32>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let fortress = request::fortress::get(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortress.wood))
}

/// # Errors
///
/// Will return `Err` if the get or patch failed.
#[utoipa::path(
    get,
    path = "/{fortress_id}/wood/collect",
    tag = FORTRESS_TAG,
    responses(
        (status = 200, description = "Collect more wood", body = Fortress)
    )
)]
pub async fn fortress_wood_collect(
    Path(fortress_id): Path<i32>,
) -> Result<Json<Fortress>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let fortress = request::fortress::get(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    // TODO calculer le nouveau montant de gold avec un truc bien
    let update_fortress = UpdateFortress {
        gold: None,
        food: None,
        wood: Some(fortress.wood + 1),
        energy: None,
    };
    let fortress = request::fortress::patch(&client, &api_url, fortress_id, &update_fortress)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortress))
}

/// # Errors
///
/// Will return `Err` if the get failed.
#[utoipa::path(
    get,
    path = "/{fortress_id}/energy",
    tag = FORTRESS_TAG,
    responses(
        (status = 200, description = "get energy of fortress", body = i32)
    )
)]
pub async fn fortress_energy_get(
    Path(fortress_id): Path<i32>,
) -> Result<Json<i32>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let fortress = request::fortress::get(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortress.energy))
}

/// # Errors
///
/// Will return `Err` if the get or patch failed.
#[utoipa::path(
    get,
    path = "/{fortress_id}/energy/collect",
    tag = FORTRESS_TAG,
    responses(
        (status = 200, description = "Collect more energy", body = Fortress)
    )
)]
pub async fn fortress_energy_collect(
    Path(fortress_id): Path<i32>,
) -> Result<Json<Fortress>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let fortress = request::fortress::get(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    // TODO calculer le nouveau montant de gold avec un truc bien
    let update_fortress = UpdateFortress {
        gold: None,
        food: None,
        wood: None,
        energy: Some(fortress.energy + 1),
    };
    let fortress = request::fortress::patch(&client, &api_url, fortress_id, &update_fortress)
        .await
        .map_err(internal_error)?;
    Ok(Json(fortress))
}

/// # Errors
///
/// Will return `Err` if the get failed.
#[utoipa::path(
    get,
    path = "/{fortress_id}/building",
    tag = FORTRESS_TAG,
    responses(
        (status = 200, description = "Get all building on fortress", body = [Building])
    )
)]
pub async fn fortress_building_get_all(
    Path(fortress_id): Path<i32>,
) -> Result<Json<Vec<Building>>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let buildings = request::building::get_by_fortress(&client, &api_url, fortress_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(buildings))
}

/// # Errors
///
/// Will return `Err` if the get failed.
#[utoipa::path(
    get,
    path = "",
    tag = BUILDING_TAG,
    responses(
        (status = 200, description = "List all building", body = [Building])
    )
)]
pub async fn building_get_all() -> Result<Json<Vec<Building>>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let buildings = request::building::get_all(&client, &api_url)
        .await
        .map_err(internal_error)?;
    Ok(Json(buildings))
}

/// # Errors
///
/// Will return `Err` if the get failed.
#[utoipa::path(
    get,
    path = "/{building_id}",
    tag = BUILDING_TAG,
    responses(
        (status = 200, description = "Get one building", body = Building)
    )
)]
pub async fn building_get(
    Path(building_id): Path<i32>,
) -> Result<Json<Building>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let building = request::building::get(&client, &api_url, building_id)
        .await
        .map_err(internal_error)?;
    Ok(Json(building))
}

/// # Errors
///
/// Will return `Err` if the get or patch failed.
#[utoipa::path(
    get,
    path = "/{building_id}/improve",
    tag = BUILDING_TAG,
    responses(
        (status = 200, description = "Improve one building", body = Building)
    )
)]
pub async fn building_improve(
    Path(building_id): Path<i32>,
) -> Result<Json<Building>, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let api_url = std::env::var("CRUD_SERVER_URL").map_err(internal_error)?;
    let building = request::building::get(&client, &api_url, building_id)
        .await
        .map_err(internal_error)?;
    // TODO: trouver une meilleure methode (consomation de ressource, etc ...)
    let update_building = UpdateBuilding {
        name: None,
        level: Some(building.level + 1),
        fortress_id: None,
    };
    let building = request::building::patch(&client, &api_url, building_id, &update_building)
        .await
        .map_err(internal_error)?;
    Ok(Json(building))
}
