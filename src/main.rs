use core::fmt;
use std::io::{self, stdin, Write};

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
}

impl fmt::Display for Fortress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "gold: {}\nfood: {}\nwood: {}\nenergy: {}",
            self.gold, self.food, self.wood, self.energy
        )
    }
}

fn print_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

fn print_help() {
    println!("0: help\n1: stats\n2: to earn gold\n3: to gather food\n4: to fell trees\n5: to collect energy\n6: \n7: improve farm\n9: exit");
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
                1 => println!("{fortress}"),
                2 => fortress.earn_gold(),
                3 => fortress.gather_food(),
                4 => fortress.fell_trees(),
                5 => fortress.collect_energy(),
                7 => fortress.improve_farm(),
                9 => break,
                _ => {}
            },
            Err(e) => println!("{e}"),
        }
        print_prompt();
    }
}
