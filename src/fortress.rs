use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};
use std::{collections::HashMap, fs};

#[derive(Deserialize, Serialize, Debug)]
struct Building {
    name: String,
    level: i32,
}

impl Building {
    pub fn from(name: &str) -> Self {
        Self {
            name: String::from(name),
            level: 1,
        }
    }

    pub fn improve(&mut self) {
        self.level += 1;
    }

    pub fn level(&self) -> i32 {
        self.level
    }
}

fn default_buildings() -> HashMap<String, Building> {
    HashMap::from([
        ("farm".to_string(), Building::from("farm")),
        ("sawmill".to_string(), Building::from("sawmill")),
        ("sanctuary".to_string(), Building::from("sanctuary")),
    ])
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Fortress {
    gold: i32,
    food: i32,
    wood: i32,
    energy: i32,
    buildings: HashMap<String, Building>,
}

impl Default for Fortress {
    fn default() -> Self {
        Self::new()
    }
}

impl Fortress {
    #[must_use]
    pub fn new() -> Self {
        Self {
            gold: 0,
            food: 0,
            wood: 0,
            energy: 0,
            buildings: default_buildings(),
        }
    }

    pub fn print_stats(&self) {
        println!("{:?}", *self);
    }

    fn use_energy(&mut self, cost: i32) -> Result<(), ()> {
        if self.energy >= cost {
            self.energy -= cost;
            Ok(())
        } else {
            println!("Not enough energy: {} < {cost}", self.energy);
            Err(())
        }
    }

    fn use_wood(&mut self, cost: i32) -> Result<(), ()> {
        if self.wood >= cost {
            self.wood -= cost;
            Ok(())
        } else {
            println!("Not enough wood: {} < {cost}", self.wood);
            Err(())
        }
    }

    pub fn earn_gold(&mut self) {
        if self.use_energy(1).is_ok() {
            println!("The character ventures out to earn gold.");
            self.gold += 1;
        }
    }

    pub fn gather_food(&mut self) {
        if self.use_energy(1).is_ok() {
            println!("The character seeks to gather food.");
            self.food += self.buildings["farm"].level();
        }
    }

    pub fn fell_trees(&mut self) {
        if self.use_energy(1).is_ok() {
            println!("The character is going to chop wood.");
            self.wood += self.buildings["sawmill"].level();
        }
    }

    pub fn collect_energy(&mut self) {
        println!("The character sets out to collect energy.");
        self.energy += self.buildings["sanctuary"].level();
    }

    pub fn improve(&mut self, building_name: &str) {
        if self.buildings.contains_key(building_name) {
            if self
                .use_wood(self.buildings[building_name].level() * 2)
                .is_ok()
            {
                if let Some(building) = self.buildings.get_mut(building_name) {
                    building.improve();
                    println!("\"{building_name}\" level increased!");
                }
            }
        } else {
            println!("\"{building_name}\" not found.");
        }
    }

    pub fn save(&self, save_path: &str) {
        let fortress = json!(self);

        if fs::write(save_path, fortress.to_string()).is_ok() {
            println!("save success");
        } else {
            println!("save failed");
        }
    }

    pub fn load(&mut self, save_path: &str) {
        if let Ok(data) = fs::read_to_string(save_path) {
            if let Ok(fortress) = from_str::<Fortress>(&data) {
                *self = fortress;
                for building in default_buildings() {
                    self.buildings.entry(building.0).or_insert(building.1);
                }
                println!("Load successful!");
            } else {
                println!("Backup data recovery failed");
            }
        } else {
            println!("Reading file {save_path} failed");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let fortress = Fortress::new();

        fortress.print_stats();
    }

    #[test]
    fn test_earn_without_energy() {
        let mut fortress = Fortress::new();

        assert_eq!(fortress.gold, 0);
        assert_eq!(fortress.food, 0);
        assert_eq!(fortress.wood, 0);
        assert_eq!(fortress.energy, 0);
        fortress.earn_gold();
        fortress.gather_food();
        fortress.fell_trees();
        assert_eq!(fortress.gold, 0);
        assert_eq!(fortress.food, 0);
        assert_eq!(fortress.wood, 0);
        assert_eq!(fortress.energy, 0);
    }

    #[test]
    fn test_earn_with_energy() {
        let mut fortress = Fortress::new();

        assert_eq!(fortress.energy, 0);
        fortress.collect_energy();
        fortress.collect_energy();
        fortress.collect_energy();
        assert_eq!(fortress.energy, 3);
        fortress.earn_gold();
        fortress.gather_food();
        fortress.fell_trees();
        assert_eq!(fortress.gold, 1);
        assert_eq!(fortress.food, 1);
        assert_eq!(fortress.wood, 1);
        assert_eq!(fortress.energy, 0);
    }

    #[test]
    fn test_improve_farm() {
        let mut fortress = Fortress::new();

        assert_eq!(fortress.buildings["farm"].level, 1);
        fortress.improve("farm");
        assert_eq!(fortress.buildings["farm"].level, 1);
        fortress.wood = 3;
        fortress.improve("farm");
        assert_eq!(fortress.wood, 1);
        assert_eq!(fortress.buildings["farm"].level, 2);
        fortress.improve("farm");
        assert_eq!(fortress.wood, 1);
        assert_eq!(fortress.buildings["farm"].level, 2);
        fortress.wood = 4;
        fortress.improve("farm");
        assert_eq!(fortress.wood, 0);
        assert_eq!(fortress.buildings["farm"].level, 3);
    }

    #[test]
    fn test_improve_sawmill() {
        let mut fortress = Fortress::new();

        assert_eq!(fortress.buildings["sawmill"].level, 1);
        fortress.improve("sawmill");
        assert_eq!(fortress.buildings["sawmill"].level, 1);
        fortress.wood = 3;
        fortress.improve("sawmill");
        assert_eq!(fortress.wood, 1);
        assert_eq!(fortress.buildings["sawmill"].level, 2);
        fortress.improve("sawmill");
        assert_eq!(fortress.wood, 1);
        assert_eq!(fortress.buildings["sawmill"].level, 2);
        fortress.wood = 4;
        fortress.improve("sawmill");
        assert_eq!(fortress.wood, 0);
        assert_eq!(fortress.buildings["sawmill"].level, 3);
    }

    #[test]
    fn test_improve_sanctuary() {
        let mut fortress = Fortress::new();

        assert_eq!(fortress.buildings["sanctuary"].level, 1);
        fortress.improve("sanctuary");
        assert_eq!(fortress.buildings["sanctuary"].level, 1);
        fortress.wood = 3;
        fortress.improve("sanctuary");
        assert_eq!(fortress.wood, 1);
        assert_eq!(fortress.buildings["sanctuary"].level, 2);
        fortress.improve("sanctuary");
        assert_eq!(fortress.wood, 1);
        assert_eq!(fortress.buildings["sanctuary"].level, 2);
        fortress.wood = 4;
        fortress.improve("sanctuary");
        assert_eq!(fortress.wood, 0);
        assert_eq!(fortress.buildings["sanctuary"].level, 3);
    }

    #[test]
    fn test_improve_unknown() {
        let mut fortress = Fortress::new();

        assert!(!fortress.buildings.contains_key("unknown"));
        fortress.improve("unknown");
        assert!(!fortress.buildings.contains_key("unknown"));
        fortress.wood = 3;
        fortress.improve("unknown");
        assert_eq!(fortress.wood, 3);
        assert!(!fortress.buildings.contains_key("unknown"));
        fortress.improve("unknown");
        assert_eq!(fortress.wood, 3);
        assert!(!fortress.buildings.contains_key("unknown"));
        fortress.wood = 4;
        fortress.improve("unknown");
        assert_eq!(fortress.wood, 4);
        assert!(!fortress.buildings.contains_key("unknown"));
    }

    #[test]
    fn test_load() {
        let mut fortress = Fortress::new();
        let data = r#"{"buildings":{"farm":{"level":1,"name":"super farm"},"sanctuary":{"level":2,"name":"super sanctuary"},"sawmill":{"level":3,"name":"super sawmill"}},"energy":4,"food":5,"gold":6,"wood":7}"#;
        assert!(fs::write("test-save.json", data).is_ok());
        fortress.load("test-save.json");
        assert!(fs::remove_file("test-save.json").is_ok());
        assert_eq!(fortress.gold, 6);
        assert_eq!(fortress.food, 5);
        assert_eq!(fortress.wood, 7);
        assert_eq!(fortress.energy, 4);
        assert_eq!(fortress.buildings["farm"].name, "super farm");
        assert_eq!(fortress.buildings["farm"].level, 1);
        assert_eq!(fortress.buildings["sanctuary"].name, "super sanctuary");
        assert_eq!(fortress.buildings["sanctuary"].level, 2);
        assert_eq!(fortress.buildings["sawmill"].name, "super sawmill");
        assert_eq!(fortress.buildings["sawmill"].level, 3);
    }

    #[test]
    fn test_load_incomplete_save_0() {
        let mut fortress = Fortress::new();
        let data = r#"{"buildings":{"farm":{"level":42,"name":"super farm"}},"energy":4,"food":2,"gold":1,"wood":3}"#;
        assert!(fs::write("test-incomplete-save-0.json", data).is_ok());
        fortress.load("test-incomplete-save-0.json");
        assert!(fs::remove_file("test-incomplete-save-0.json").is_ok());
        assert_eq!(fortress.gold, 1);
        assert_eq!(fortress.food, 2);
        assert_eq!(fortress.wood, 3);
        assert_eq!(fortress.energy, 4);
        assert_eq!(fortress.buildings["farm"].name, "super farm");
        assert_eq!(fortress.buildings["farm"].level, 42);
        assert_eq!(fortress.buildings["sanctuary"].name, "sanctuary");
        assert_eq!(fortress.buildings["sanctuary"].level, 1);
        assert_eq!(fortress.buildings["sawmill"].name, "sawmill");
        assert_eq!(fortress.buildings["sawmill"].level, 1);
    }

    #[test]
    fn test_load_incomplete_save_1() {
        let mut fortress = Fortress::new();
        let data = r#"{"buildings":{},"energy":4,"food":2,"gold":1,"wood":3}"#;
        assert!(fs::write("test-incomplete-save-1.json", data).is_ok());
        fortress.load("test-incomplete-save-1.json");
        assert!(fs::remove_file("test-incomplete-save-1.json").is_ok());
        assert_eq!(fortress.gold, 1);
        assert_eq!(fortress.food, 2);
        assert_eq!(fortress.wood, 3);
        assert_eq!(fortress.energy, 4);
        assert_eq!(fortress.buildings["farm"].name, "farm");
        assert_eq!(fortress.buildings["farm"].level, 1);
        assert_eq!(fortress.buildings["sanctuary"].name, "sanctuary");
        assert_eq!(fortress.buildings["sanctuary"].level, 1);
        assert_eq!(fortress.buildings["sawmill"].name, "sawmill");
        assert_eq!(fortress.buildings["sawmill"].level, 1);
    }
}
