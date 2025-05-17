use models::{Building, Fortress, NewBuilding, NewFortress, UpdateBuilding, UpdateFortress};

pub mod models;
pub mod request;
pub mod schema;

// TODO: create a `Resources` structure and refactor this with `Fortress` resources
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Costs {
    pub gold: i32,
    pub food: i32,
    pub wood: i32,
    pub energy: i32,
}

/// # Errors
///
/// Will return `Err` if the improve failed.
pub fn upgrade_building(
    fortress: &Fortress,
    building: &Building,
    costs: &Costs,
    max_building_level: i32,
) -> Result<(UpdateFortress, UpdateBuilding), String> {
    if fortress.id != building.fortress_id {
        return Err(String::from("fortress id mismatch"));
    }
    if building.level >= max_building_level {
        return Err(String::from("building level is too high"));
    }
    if fortress.gold < costs.gold
        || fortress.food < costs.food
        || fortress.wood < costs.wood
        || fortress.energy < costs.energy
    {
        let error = format!("insufficient resources: {fortress:?} {costs:?}");
        return Err(error);
    }
    let update_fortress = UpdateFortress {
        gold: Some(fortress.gold - costs.gold),
        food: Some(fortress.food - costs.food),
        wood: Some(fortress.wood - costs.wood),
        energy: Some(fortress.energy - costs.energy),
    };
    let update_building = UpdateBuilding {
        name: None,
        level: Some(building.level + 1),
        fortress_id: None,
    };
    Ok((update_fortress, update_building))
}

#[must_use]
pub fn upgrade_cost(level: i32, base: i32, factor: f64) -> f64 {
    let level = if level < 1 { 1 } else { level };
    f64::from(base) * factor.powf(f64::from(level - 1))
}

#[must_use]
pub fn optimize_factor(level_max: i32, base: i32, cost_max: i32) -> f64 {
    (f64::from(cost_max) / f64::from(base)).powf(1.0 / (f64::from(level_max) - 1.0))
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn get_costs(level: i32, level_max: i32) -> Costs {
    let factor = optimize_factor(level_max, 10, i32::MAX);
    let cost = upgrade_cost(level, 10, factor) as i32;
    Costs {
        gold: cost / 2,
        food: cost,
        wood: cost / 5,
        energy: cost / 10,
    }
}

#[must_use]
pub fn get_gold_bonus(buildings: &[Building]) -> i32 {
    let mut gold_bonus = 0;

    for building in buildings {
        if building.name == "bank" {
            gold_bonus += building.level;
        }
    }
    gold_bonus
}

#[must_use]
pub fn get_food_bonus(buildings: &[Building]) -> i32 {
    let mut food_bonus = 0;

    for building in buildings {
        if building.name == "farm" {
            food_bonus += building.level;
        }
    }
    food_bonus
}

#[must_use]
pub fn get_wood_bonus(buildings: &[Building]) -> i32 {
    let mut wood_bonus = 0;

    for building in buildings {
        if building.name == "sawmill" {
            wood_bonus += building.level;
        }
    }
    wood_bonus
}

#[must_use]
pub fn get_energy_bonus(buildings: &[Building]) -> i32 {
    let mut energy_bonus = 0;

    for building in buildings {
        if building.name == "sanctuary" {
            energy_bonus += building.level;
        }
    }
    energy_bonus
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upgrade_building() {
        let fortress = Fortress {
            id: 0,
            gold: 0,
            food: 0,
            wood: 0,
            energy: 0,
        };
        let building = Building {
            id: 0,
            name: String::from(""),
            level: 1,
            fortress_id: 0,
        };
        let costs = Costs {
            gold: 0,
            food: 0,
            wood: 0,
            energy: 0,
        };
        let result = upgrade_building(&fortress, &building, &costs, 200);
        assert!(result.is_ok());
        if let Ok((update_fortress, update_building)) = result {
            assert_eq!(update_fortress.gold, Some(0));
            assert_eq!(update_fortress.food, Some(0));
            assert_eq!(update_fortress.wood, Some(0));
            assert_eq!(update_fortress.energy, Some(0));
            assert_eq!(update_building.level, Some(building.level + 1));
        }
    }

    #[test]
    fn test_upgrade_building_error() {
        {
            let fortress = Fortress {
                id: 0,
                gold: 10,
                food: 0,
                wood: 0,
                energy: 0,
            };
            let building = Building {
                id: 0,
                name: String::from(""),
                level: 1,
                fortress_id: 0,
            };
            let costs = Costs {
                gold: 11,
                food: 0,
                wood: 0,
                energy: 0,
            };
            let result = upgrade_building(&fortress, &building, &costs, 200);
            assert!(result.is_err());
            if let Err(msg) = result {
                println!("error: {msg}");
            }
        }
        {
            let fortress = Fortress {
                id: 0,
                gold: 10,
                food: 0,
                wood: 0,
                energy: 0,
            };
            let building = Building {
                id: 0,
                name: String::from(""),
                level: 200,
                fortress_id: 0,
            };
            let costs = Costs {
                gold: 5,
                food: 0,
                wood: 0,
                energy: 0,
            };
            let result = upgrade_building(&fortress, &building, &costs, 200);
            assert!(result.is_err());
            if let Err(msg) = result {
                println!("error: {msg}");
            }
        }
        {
            let fortress = Fortress {
                id: 1,
                gold: 10,
                food: 0,
                wood: 0,
                energy: 0,
            };
            let building = Building {
                id: 0,
                name: String::from(""),
                level: 1,
                fortress_id: 0,
            };
            let costs = Costs {
                gold: 5,
                food: 0,
                wood: 0,
                energy: 0,
            };
            let result = upgrade_building(&fortress, &building, &costs, 200);
            assert!(result.is_err());
            if let Err(msg) = result {
                println!("error: {msg}");
            }
        }
    }

    #[test]
    fn test_optimize_factor() {
        const BASE: i32 = 10;
        let cost_max: i32 = i32::MAX;
        let levels: Vec<i32> = vec![10, 50, 100, 200, 1000];

        for level in levels {
            let factor = optimize_factor(level, BASE, cost_max);
            println!("level max: {level}, base: {BASE}, factor: {factor}");
            assert!(factor >= 1.0);
        }
    }

    #[test]
    fn test_upgrade_cost() {
        const BASE: i32 = 10;
        const FACTOR: f64 = 1.1012;
        let result = upgrade_cost(0, BASE, FACTOR) as i32;
        assert_eq!(result, 10);
        let result = upgrade_cost(1, BASE, FACTOR) as i32;
        assert_eq!(result, 10);
        let result = upgrade_cost(2, BASE, FACTOR) as i32;
        assert_eq!(result, 11);
        let result = upgrade_cost(3, BASE, FACTOR) as i32;
        assert_eq!(result, 12);
        let result = upgrade_cost(4, BASE, FACTOR) as i32;
        assert_eq!(result, 13);
        let result = upgrade_cost(5, BASE, FACTOR) as i32;
        assert_eq!(result, 14);
        let result = upgrade_cost(50, BASE, FACTOR) as i32;
        assert_eq!(result, 1125);
        let result = upgrade_cost(100, BASE, FACTOR) as i32;
        assert_eq!(result, 139557);
        let result = upgrade_cost(150, BASE, FACTOR) as i32;
        assert_eq!(result, 17300721);
        let result = upgrade_cost(200, BASE, FACTOR) as i32;
        assert_eq!(result, 2144738468);
    }

    #[test]
    fn test_upgrade_cost_overflow() {
        const BASE: i32 = 10;
        const FACTOR: f64 = 1.1012;
        let result = upgrade_cost(250, BASE, FACTOR) as i32;
        assert_eq!(result, i32::MAX);
        let result = upgrade_cost(1000, BASE, FACTOR) as i32;
        assert_eq!(result, i32::MAX);
        let result = upgrade_cost(i32::MAX, BASE, FACTOR) as i32;
        assert_eq!(result, i32::MAX);
    }

    #[test]
    fn test_level_slice() {
        const BASE: i32 = 10;
        let cost_max: i32 = i32::MAX;
        let levels: Vec<i32> = vec![10, 50, 100, 200, 1000];

        for level in levels {
            let factor = optimize_factor(level, BASE, cost_max);
            let result_min = upgrade_cost(1, BASE, factor) as i32;
            let result_max = upgrade_cost(level, BASE, factor) as i32;
            println!("{}: {} -> {}: {}", 1, result_min, level, result_max);
            assert!(BASE <= result_min && result_min <= cost_max);
            assert!(BASE <= result_max && result_max <= cost_max);
            assert!(result_min <= result_max);
        }
    }
}
