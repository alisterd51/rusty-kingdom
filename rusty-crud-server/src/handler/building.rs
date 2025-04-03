use crate::AppState;
use super::internal_error;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rusty::{
    models::{Building, NewBuilding, UpdateBuilding},
    schema::buildings,
};

/// # Errors
///
/// Will return `Err` if the insert failed.
pub async fn post(
    State(app_state): State<AppState>,
    Json(new_building): Json<NewBuilding>,
) -> Result<Json<Building>, (StatusCode, String)> {
    let mut conn = app_state.diesel_pool.get().await.map_err(internal_error)?;
    let res = diesel::insert_into(buildings::table)
        .values(new_building)
        .returning(Building::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(res))
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get_all(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Building>>, (StatusCode, String)> {
    let mut conn = app_state.diesel_pool.get().await.map_err(internal_error)?;
    let res = buildings::table
        .get_results(&mut conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(res))
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get(
    State(app_state): State<AppState>,
    Path(building_id): Path<i32>,
) -> Result<Json<Building>, (StatusCode, String)> {
    let mut redis_conn = app_state.redis_client
        .get_multiplexed_async_connection().await.map_err(internal_error)?;
    let redis_key = format!("building:{building_id}");
    let res_cached: Option<String> = redis::cmd("GET")
        .arg(&redis_key)
        .query_async(&mut redis_conn)
        .await
        .map_err(internal_error)?;
    if let Some(res_cached) = res_cached {
        let res = serde_json::from_str(&res_cached).map_err(internal_error)?;
        return Ok(Json(res));
    }

    let mut diesel_conn = app_state.diesel_pool.get().await.map_err(internal_error)?;
    let res = buildings::table
        .filter(buildings::id.eq(building_id))
        .get_result(&mut diesel_conn)
        .await
        .map_err(internal_error)?;

    let redis_value = serde_json::to_string(&res).map_err(internal_error)?;

    redis::cmd("SET")
        .arg(&redis_key)
        .arg(&redis_value)
        .exec_async(&mut redis_conn)
        .await
        .map_err(internal_error)?;

    Ok(Json(res))
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get_by_fortress(
    State(app_state): State<AppState>,
    Path(fortress_id): Path<i32>,
) -> Result<Json<Vec<Building>>, (StatusCode, String)> {
    let mut conn = app_state.diesel_pool.get().await.map_err(internal_error)?;
    let res = buildings::table
        .filter(buildings::fortress_id.eq(fortress_id))
        .get_results(&mut conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(res))
}

/// # Errors
///
/// Will return `Err` if the update failed.
pub async fn patch(
    State(app_state): State<AppState>,
    Path(building_id): Path<i32>,
    Json(update_building): Json<UpdateBuilding>,
) -> Result<Json<Building>, (StatusCode, String)> {
    let mut conn = app_state.diesel_pool.get().await.map_err(internal_error)?;
    let res = diesel::update(buildings::table)
        .filter(buildings::id.eq(building_id))
        .set(update_building)
        .returning(Building::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(res))
}

/// # Errors
///
/// Will return `Err` if the delete failed.
pub async fn delete(
    State(app_state): State<AppState>,
    Path(building_id): Path<i32>,
) -> Result<Json<usize>, (StatusCode, String)> {
    let mut conn = app_state.diesel_pool.get().await.map_err(internal_error)?;
    let res = diesel::delete(buildings::table)
        .filter(buildings::id.eq(building_id))
        .execute(&mut conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(res))
}

/// # Errors
///
/// Will return `Err` if the delete failed.
pub async fn delete_by_fortress(
    State(app_state): State<AppState>,
    Path(fortress_id): Path<i32>,
) -> Result<Json<usize>, (StatusCode, String)> {
    let mut conn = app_state.diesel_pool.get().await.map_err(internal_error)?;
    let res = diesel::delete(buildings::table)
        .filter(buildings::fortress_id.eq(fortress_id))
        .execute(&mut conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(res))
}
