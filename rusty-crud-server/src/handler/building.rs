use super::{Pool, internal_error};
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
    State(pool): State<Pool>,
    Json(new_building): Json<NewBuilding>,
) -> Result<Json<Building>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;
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
    State(pool): State<Pool>,
) -> Result<Json<Vec<Building>>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;
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
    State(pool): State<Pool>,
    Path(building_id): Path<i32>,
) -> Result<Json<Building>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;
    let res = buildings::table
        .filter(buildings::id.eq(building_id))
        .get_result(&mut conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(res))
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get_by_fortress(
    State(pool): State<Pool>,
    Path(fortress_id): Path<i32>,
) -> Result<Json<Vec<Building>>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;
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
    State(pool): State<Pool>,
    Path(building_id): Path<i32>,
    Json(update_building): Json<UpdateBuilding>,
) -> Result<Json<Building>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;
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
    State(pool): State<Pool>,
    Path(building_id): Path<i32>,
) -> Result<Json<usize>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;
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
    State(pool): State<Pool>,
    Path(fortress_id): Path<i32>,
) -> Result<Json<usize>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;
    let res = diesel::delete(buildings::table)
        .filter(buildings::fortress_id.eq(fortress_id))
        .execute(&mut conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(res))
}
