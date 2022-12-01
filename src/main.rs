

// This declaration will look for a file named `days.rs` and will
// insert its contents inside a module named `days` under this scope
pub mod days;
pub mod utils;

fn main() {
    println!("Day1");
    println!("Part1: {}", days::day1::solve_part_1());
    println!("Part2: {}", days::day1::solve_part_2());
    println!("Day2");
    println!("Part1: {}", days::day2::solve_part_1());
    println!("Part2: {}", days::day2::solve_part_2());
}
