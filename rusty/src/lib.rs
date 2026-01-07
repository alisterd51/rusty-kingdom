use models::{NewBuilding, NewFortress};

pub mod models;
pub mod schema;

// TODO: create a `Resources` structure and refactor this with `Fortress` resources
pub struct Costs {
    pub gold: i32,
    pub food: i32,
    pub wood: i32,
    pub energy: i32,
}

impl Default for NewFortress {
    fn default() -> Self {
        Self::new()
    }
}

impl NewFortress {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            gold: 0,
            food: 0,
            wood: 0,
            energy: 0,
        }
    }
}

impl NewBuilding {
    #[must_use]
    pub const fn new(name: String, fortress_id: i32) -> Self {
        Self {
            name,
            level: 0,
            fortress_id,
        }
    }
}
