
pub fn solve_part_1() -> String {
    let calories: Vec<i32> = get_calories();
    let result1:String = match calories.iter().max() {
        Some(max_value) => max_value.to_string(),
        None => "Vector is empty".to_string(),
    };
    result1
}

pub fn solve_part_2() -> String {
    let mut calories: Vec<i32> = get_calories();
    let mut result2:i32 = 0;
    for _i in 1..=3 {
        if let Some(max_value) = calories.iter().max() {
            if let Some(index) = calories.iter().position(|&i| i == *max_value) {
                result2 += calories[index]; // max_value == calories[index]
                calories.swap_remove(index);
            }
        }
    }
    result2.to_string()
}

fn get_calories() -> Vec<i32> {
    let list = crate::load_input_list(1);
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
    calories
}
