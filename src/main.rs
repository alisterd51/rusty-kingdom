#![forbid(unsafe_code)]

mod fortress;

use fortress::Fortress;
use std::io::{self, stdin, Write};

fn print_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

fn print_help() {
    println!("0: help\n1: stats\n2: to earn gold\n3: to gather food\n4: to fell trees\n5: to collect energy\n6: \n7: improve farm\n8: improve sawmill\n9: improve sanctuary\n10: save game\n11: load game\n12: exit");
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
                7 => fortress.improve("farm"),
                8 => fortress.improve("sawmill"),
                9 => fortress.improve("sanctuary"),
                10 => fortress.save("save.json"),
                11 => fortress.load("save.json"),
                12 => break,
                _ => {}
            },
            Err(e) => println!("{e}"),
        }
        print_prompt();
    }
}
