use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    println!("Day1");
    let list = load_input_list(1);
    let mut calories: Vec<i32> = Vec::new();
    let mut current_sum: i32 = 0;
    for line in list {
        if line == "" {
            calories.push(current_sum);
            current_sum = 0;
            continue;
        }
        current_sum += line.parse::<i32>().unwrap();
    }
    let result1:String = match calories.iter().max() {
        Some(maxValue) => maxValue.to_string(),
        None => "Vector is empty".to_string(),
    };

    println!("Part1");
    println!("{}\n", result1);

    let mut result2:i32 = 0;
    for i in (1..=3) {
        if let Some(max_value) = calories.iter().max() {
            if let Some(index) = calories.iter().position(|&i| i == *max_value) {
                result2 += calories[index]; // max_value == calories[index]
                calories.swap_remove(index);
            }
        }
    }

    println!("Part2");
    println!("{}\n", result2.to_string());
}

fn load_input_list(day_nr: i32) -> Vec<String> {
    let file_path = format!("input/day{day_nr}.txt");
    let file = File::open(file_path).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
