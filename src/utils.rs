use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn load_input_list(day_nr: i32) -> Vec<String> {
    let file_path = format!("input/day{day_nr}.txt");
    let file = File::open(file_path).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
