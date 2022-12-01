use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// This declaration will look for a file named `days.rs` and will
// insert its contents inside a module named `days` under this scope
pub mod days;

fn main() {
    println!("Day1");
    println!("Part1: {}", days::day1::solve_part_1());
    println!("Part2: {}", days::day1::solve_part_2());
}

fn load_input_list(day_nr: i32) -> Vec<String> {
    let file_path = format!("input/day{day_nr}.txt");
    let file = File::open(file_path).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
