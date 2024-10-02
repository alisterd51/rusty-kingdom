use core::fmt;
use std::io::{self, stdin, Write};

struct Resources {
    gold: i32,
    food: i32,
    wood: i32,
    energy: i32,
}

impl Resources {
    pub const fn new() -> Self {
        Self {
            gold: 0,
            food: 0,
            wood: 0,
            energy: 0,
        }
    }

    fn to_earn_gold(&mut self) {
        println!("The character ventures out to earn gold.");
        self.gold += 1;
    }

    fn to_gather_food(&mut self) {
        println!("The character seeks to gather food.");
        self.food += 1;
    }

    fn to_fell_trees(&mut self) {
        println!("The character is going to chop wood.");
        self.wood += 1;
    }

    fn to_collect_energy(&mut self) {
        println!("The character sets out to collect energy.");
        self.energy += 1;
    }
}

impl fmt::Display for Resources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "gold: {}\nfood: {}\nwood: {}\nenergy: {}", self.gold, self.food, self.wood, self.energy)
    }
}

fn print_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

fn print_help() {
    println!("0: help\n1: stats\n2: to earn gold\n3: to gather food\n4: to fell trees\n5: to collect energy\n9: exit");
}

fn main() {
    let mut resources = Resources::new();

    print_help();
    print_prompt();
    for line in stdin().lines() {
        let line = line.unwrap();

        match line.trim().parse() {
            Ok(n) => {
                match n {
                    0 => print_help(),
                    1 => println!("{}", resources),
                    2 => resources.to_earn_gold(),
                    3 => resources.to_gather_food(),
                    4 => resources.to_fell_trees(),
                    5 => resources.to_collect_energy(),
                    9 => break,
                    _ => {},
                }
            },
            Err(e) => println!("{}", e),
        }
        print_prompt();
    }
}
