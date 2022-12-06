use std::env;
use chrono::prelude::*;

// This declaration will look for a file named `days.rs` and will
// insert its contents inside a module named `days` under this scope
pub mod days;
pub mod utils;

fn main() {
    let day:u32 = evaluate_args(env::args().collect());

    match day {
        1 => {
            println!("Day 1");
            println!("Part1: {}", days::day1::solve_part_1());
            println!("Part2: {}", days::day1::solve_part_2());
        },
        2 => {
            println!("Day 2");
            println!("Part1: {}", days::day2::solve_part_1());
            println!("Part2: {}", days::day2::solve_part_2());
        },
        3 => {
            println!("Day 3");
            println!("Part1: {}", days::day3::solve_part_1());
            println!("Part2: {}", days::day3::solve_part_2());
        },
        4 => {
            println!("Day 4");
            println!("Part1: {}", days::day4::solve_part_1());
            println!("Part2: {}", days::day4::solve_part_2());
        },
        5 => {
            println!("Day 5");
            println!("Part1: {}", days::day5::solve_part_1());
            println!("Part2: {}", days::day5::solve_part_2());
        },
        _ => {
            println!("Not yet implemented!");
        },
    }
}

fn evaluate_args(args: Vec<String>) -> u32 {
    let mut day:u32 = 0;
    if args.len() > 1 {
        day = args[1].parse::<u32>().unwrap();
    }

    // if nothing else is specified, run the current day
    if day == 0 {
        day = Local::now().day();
    }

    day
}
