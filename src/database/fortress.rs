use crate::{
    models::{Fortress, NewFortress, UpdateFortress},
    schema::fortresses,
};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

/// # Errors
///
/// Will return `Err` if the insert failed.
pub fn create(
    conn: &mut PgConnection,
    new_fortress: &NewFortress,
) -> Result<Fortress, Box<dyn std::error::Error>> {
    let fortress = diesel::insert_into(fortresses::table)
        .values(new_fortress)
        .returning(Fortress::as_returning())
        .get_result(conn)?;
    Ok(fortress)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub fn get_all(conn: &mut PgConnection) -> Result<Vec<Fortress>, Box<dyn std::error::Error>> {
    let fortresses = fortresses::table.get_results(conn)?;
    Ok(fortresses)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub fn get(conn: &mut PgConnection, id: i32) -> Result<Fortress, Box<dyn std::error::Error>> {
    let fortress = fortresses::table
        .filter(fortresses::id.eq(id))
        .get_result(conn)?;
    Ok(fortress)
}

/// # Errors
///
/// Will return `Err` if the update failed.
pub fn update(
    conn: &mut PgConnection,
    id: i32,
    update_fortress: &UpdateFortress,
) -> Result<Fortress, Box<dyn std::error::Error>> {
    let result = diesel::update(fortresses::table)
        .filter(fortresses::id.eq(id))
        .set(update_fortress)
        .returning(Fortress::as_returning())
        .get_result(conn)?;
    Ok(result)
}

/// # Errors
///
/// Will return `Err` if the delete failed.
pub fn delete(conn: &mut PgConnection, id: i32) -> Result<usize, Box<dyn std::error::Error>> {
    let result = diesel::delete(fortresses::table)
        .filter(fortresses::id.eq(id))
        .execute(conn)?;
    Ok(result)
}
