use super::internal_error;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use diesel::{ExpressionMethods, RunQueryDsl, SelectableHelper, query_dsl::methods::FilterDsl};
use rusty::{
    models::{Fortress, NewFortress, UpdateFortress},
    schema::{buildings, fortresses},
};

/// # Errors
///
/// Will return `Err` if the insert failed.
pub async fn post(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(new_fortress): Json<NewFortress>,
) -> Result<Json<Fortress>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(move |conn| {
            diesel::insert_into(fortresses::table)
                .values(new_fortress)
                .returning(Fortress::as_returning())
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
) -> Result<Json<Vec<Fortress>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| fortresses::table.get_results(conn))
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
    Path(fortress_id): Path<i32>,
) -> Result<Json<Fortress>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(move |conn| {
            fortresses::table
                .filter(fortresses::id.eq(fortress_id))
                .get_result(conn)
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
    Path(fortress_id): Path<i32>,
    Json(update_building): Json<UpdateFortress>,
) -> Result<Json<Fortress>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(move |conn| {
            diesel::update(fortresses::table)
                .filter(fortresses::id.eq(fortress_id))
                .set(update_building)
                .returning(Fortress::as_returning())
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
    Path(fortress_id): Path<i32>,
) -> Result<Json<usize>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let _ = conn
        .interact(move |conn| {
            diesel::delete(buildings::table)
                .filter(buildings::fortress_id.eq(fortress_id))
                .execute(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    let res = conn
        .interact(move |conn| {
            diesel::delete(fortresses::table)
                .filter(fortresses::id.eq(fortress_id))
                .execute(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}
