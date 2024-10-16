use crate::{
    models::{Building, NewBuilding, UpdateBuilding},
    schema::buildings,
};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

/// # Errors
///
/// Will return `Err` if the insert failed.
pub fn create(
    conn: &mut PgConnection,
    new_building: &NewBuilding,
) -> Result<Building, Box<dyn std::error::Error>> {
    let building = diesel::insert_into(buildings::table)
        .values(new_building)
        .returning(Building::as_returning())
        .get_result(conn)?;
    Ok(building)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub fn get_all(conn: &mut PgConnection) -> Result<Vec<Building>, Box<dyn std::error::Error>> {
    let buildings = buildings::table.get_results(conn)?;
    Ok(buildings)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub fn get(conn: &mut PgConnection, id: i32) -> Result<Building, Box<dyn std::error::Error>> {
    let building = buildings::table
        .filter(buildings::id.eq(id))
        .get_result(conn)?;
    Ok(building)
}

/// # Errors
///
/// Will return `Err` if the update failed.
pub fn update(
    conn: &mut PgConnection,
    id: i32,
    update_building: &UpdateBuilding,
) -> Result<Building, Box<dyn std::error::Error>> {
    let result = diesel::update(buildings::table)
        .filter(buildings::id.eq(id))
        .set(update_building)
        .returning(Building::as_returning())
        .get_result(conn)?;
    Ok(result)
}

/// # Errors
///
/// Will return `Err` if the delete failed.
pub fn delete(conn: &mut PgConnection, id: i32) -> Result<usize, Box<dyn std::error::Error>> {
    let result = diesel::delete(buildings::table)
        .filter(buildings::id.eq(id))
        .execute(conn)?;
    Ok(result)
}
