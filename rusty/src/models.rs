use crate::schema::{buildings, fortresses};
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Selectable, PartialEq, Eq)]
#[diesel(table_name = fortresses)]
pub struct Fortress {
    pub id: i32,
    pub gold: i32,
    pub food: i32,
    pub wood: i32,
    pub energy: i32,
}

#[derive(Insertable)]
#[diesel(table_name = fortresses)]
pub struct NewFortress {
    pub gold: i32,
    pub food: i32,
    pub wood: i32,
    pub energy: i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = fortresses)]
pub struct UpdateFortress {
    pub gold: Option<i32>,
    pub food: Option<i32>,
    pub wood: Option<i32>,
    pub energy: Option<i32>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, PartialEq, Eq)]
#[diesel(belongs_to(Fortress))]
#[diesel(table_name = buildings)]
pub struct Building {
    pub id: i32,
    pub name: String,
    pub level: i32,
    pub fortress_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = buildings)]
pub struct NewBuilding {
    pub name: String,
    pub level: i32,
    pub fortress_id: i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = buildings)]
pub struct UpdateBuilding {
    pub name: Option<String>,
    pub level: Option<i32>,
    pub fortress_id: Option<i32>,
}
