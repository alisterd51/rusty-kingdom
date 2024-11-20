use super::internal_error;
use crate::{
    models::{Building, NewBuilding, UpdateBuilding},
    schema::buildings,
};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl, SelectableHelper};

/// # Errors
///
/// Will return `Err` if the insert failed.
pub async fn post(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(new_building): Json<NewBuilding>,
) -> Result<Json<Building>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(move |conn| {
            diesel::insert_into(buildings::table)
                .values(new_building)
                .returning(Building::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get_all(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<Building>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| buildings::table.get_results(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Path(id): Path<i32>,
) -> Result<Json<Building>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(move |conn| {
            buildings::table
                .filter(buildings::id.eq(id))
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub async fn get_by_fortress(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Path(fortress_id): Path<i32>,
) -> Result<Json<Vec<Building>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(move |conn| {
            buildings::table
                .filter(buildings::fortress_id.eq(fortress_id))
                .get_results(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

/// # Errors
///
/// Will return `Err` if the update failed.
pub async fn patch(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Path(id): Path<i32>,
    Json(update_building): Json<UpdateBuilding>,
) -> Result<Json<Building>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(move |conn| {
            diesel::update(buildings::table)
                .filter(buildings::id.eq(id))
                .set(update_building)
                .returning(Building::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

/// # Errors
///
/// Will return `Err` if the delete failed.
pub async fn delete(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Path(id): Path<i32>,
) -> Result<Json<usize>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(move |conn| {
            diesel::delete(buildings::table)
                .filter(buildings::id.eq(id))
                .execute(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

/// # Errors
///
/// Will return `Err` if the delete failed.
pub async fn delete_by_fortress(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Path(fortress_id): Path<i32>,
) -> Result<Json<usize>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(move |conn| {
            diesel::delete(buildings::table)
                .filter(buildings::fortress_id.eq(fortress_id))
                .execute(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}
