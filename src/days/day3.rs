use itertools::Itertools;
use std::collections::HashMap;

pub fn solve_part_1() -> String {
    let mut priority_sum: u32 = 0;
    let input_rows = crate::utils::load_input_list(3, "day");
    for row in input_rows {
        let left = &row[..row.len()/2].chars().sorted().unique().collect::<Vec<_>>();
        let right  =&row[row.len()/2..].chars().sorted().unique().collect::<Vec<_>>();
        let mut char_counter= HashMap::new();
        for left_char in left.into_iter() {
            char_counter.insert(left_char.to_owned(), 1);
        }
        for right_char in right.into_iter() {
            if char_counter.contains_key(&right_char) {
                priority_sum += char_to_priority(right_char);
                break;
            }
        }
    }
    priority_sum.to_string()
}

pub fn solve_part_2() -> String {
    "TODO: implement".to_string()
}

fn char_to_priority(c:&char) -> u32 {
    if c.is_lowercase() {
        return *c as u32 - 64 - 32;
    }
    *c as u32 - 64 + 26
}
