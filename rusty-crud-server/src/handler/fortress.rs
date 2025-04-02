use super::{Pool, internal_error};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rusty::{
    models::{Fortress, NewFortress, UpdateFortress},
    schema::{buildings, fortresses},
};

/// # Errors
///
/// Will return `Err` if the insert failed.
pub async fn post(
    State(pool): State<Pool>,
    Json(new_fortress): Json<NewFortress>,
) -> Result<Json<Fortress>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;
    let res = diesel::insert_into(fortresses::table)
        .values(new_fortress)
        .returning(Fortress::as_returning())
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
) -> Result<Json<Vec<Fortress>>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;
    let res = fortresses::table
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
    Path(fortress_id): Path<i32>,
) -> Result<Json<Fortress>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;
    let res = fortresses::table
        .filter(fortresses::id.eq(fortress_id))
        .get_result(&mut conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(res))
}

/// # Errors
///
/// Will return `Err` if the update failed.
pub async fn patch(
    State(pool): State<Pool>,
    Path(fortress_id): Path<i32>,
    Json(update_building): Json<UpdateFortress>,
) -> Result<Json<Fortress>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;
    let res = diesel::update(fortresses::table)
        .filter(fortresses::id.eq(fortress_id))
        .set(update_building)
        .returning(Fortress::as_returning())
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
    Path(fortress_id): Path<i32>,
) -> Result<Json<usize>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;
    let _ = diesel::delete(buildings::table)
        .filter(buildings::fortress_id.eq(fortress_id))
        .execute(&mut conn)
        .await
        .map_err(internal_error)?;
    let res = diesel::delete(fortresses::table)
        .filter(fortresses::id.eq(fortress_id))
        .execute(&mut conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(res))
}
