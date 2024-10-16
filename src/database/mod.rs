pub mod building;
pub mod fortress;

use crate::models::{Building, Fortress};
use diesel::{
    BelongingToDsl, Connection, GroupedBy, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};
use std::env;

type FortressBuildings = (Fortress, Vec<Building>);

/// # Panics
///
/// Will panic if `DATABASE_URL` is not set or invalide
#[must_use]
pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub fn get_all_fortresses_with_buildings(
    conn: &mut PgConnection,
) -> Result<Vec<FortressBuildings>, Box<dyn std::error::Error>> {
    let fortresses = fortress::get_all(conn)?;
    let buildings = building::get_all(conn)?;

    let buildings_with_fortress = buildings
        .grouped_by(&fortresses)
        .into_iter()
        .zip(fortresses)
        .map(|(buildings, fortress)| (fortress, buildings))
        .collect::<Vec<FortressBuildings>>();

    Ok(buildings_with_fortress)
}

/// # Errors
///
/// Will return `Err` if the get failed.
pub fn get_fortress_with_buildings(
    conn: &mut PgConnection,
    id: i32,
) -> Result<FortressBuildings, Box<dyn std::error::Error>> {
    let fortress = fortress::get(conn, id)?;
    let buildings = Building::belonging_to(&fortress)
        .select(Building::as_select())
        .get_results(conn)?;

    Ok((fortress, buildings))
}
