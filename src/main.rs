#![forbid(unsafe_code)]

use std::{
    fs,
    io::{self, stdin, Write},
};

use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};

#[derive(Deserialize, Serialize, Debug)]
struct Fortress {
    gold: i32,
    food: i32,
    wood: i32,
    energy: i32,
    farm_level: i32,
}

impl Fortress {
    pub const fn new() -> Self {
        Self {
            gold: 0,
            food: 0,
            wood: 0,
            energy: 0,
            farm_level: 1,
        }
    }

    fn print_stats(&self) {
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

    fn earn_gold(&mut self) {
        if self.use_energy(1).is_ok() {
            println!("The character ventures out to earn gold.");
            self.gold += 1;
        }
    }

    fn gather_food(&mut self) {
        if self.use_energy(1).is_ok() {
            println!("The character seeks to gather food.");
            self.food += self.farm_level;
        }
    }

    fn fell_trees(&mut self) {
        if self.use_energy(1).is_ok() {
            println!("The character is going to chop wood.");
            self.wood += 1;
        }
    }

    fn collect_energy(&mut self) {
        println!("The character sets out to collect energy.");
        self.energy += 1;
    }

    fn improve_farm(&mut self) {
        if self.use_wood(self.farm_level * 2).is_ok() {
            println!("farm level increased!");
            self.farm_level += 1;
        }
    }

    fn save(&self) {
        let fortress = json!(self);

        if fs::write("save.json", fortress.to_string()).is_ok() {
            println!("save success");
        } else {
            println!("save failed");
        }
    }

    fn load(&mut self) {
        if let Ok(data) = fs::read_to_string("save.json") {
            if let Ok(fortress) = from_str::<Fortress>(&data) {
                *self = fortress;
                println!("Load successful!");
            }
        }
    }
}

fn print_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

fn print_help() {
    println!("0: help\n1: stats\n2: to earn gold\n3: to gather food\n4: to fell trees\n5: to collect energy\n6: \n7: improve farm\n9: exit\n10: save game\n11: load game");
}

fn main() {
    let mut fortress = Fortress::new();

    print_help();
    print_prompt();
    for line in stdin().lines() {
        let line = line.unwrap();

        match line.trim().parse() {
            Ok(n) => match n {
                0 => print_help(),
                1 => fortress.print_stats(),
                2 => fortress.earn_gold(),
                3 => fortress.gather_food(),
                4 => fortress.fell_trees(),
                5 => fortress.collect_energy(),
                7 => fortress.improve_farm(),
                9 => break,
                10 => fortress.save(),
                11 => fortress.load(),
                _ => {}
            },
            Err(e) => println!("{e}"),
        }
        print_prompt();
    }
}
