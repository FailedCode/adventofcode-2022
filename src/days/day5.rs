use regex::Regex;

pub fn solve_part_1() -> String {
    let (stacks, commands) = get_input();
    print_stacks(&stacks);

    // for (n, from_stack, to_stack) in commands {
    //     // println!("{from_stack} -> {to_stack} ({n}x)");
    //     for _i in 0..n {
    //         let mut from_stack = stacks.get(from_stack as usize).unwrap().to_owned();
    //         let mut to_stack = stacks.get(to_stack as usize).unwrap().to_owned();
    //         let cargo_crate = from_stack.pop().unwrap();
    //         println!("moving '{cargo_crate}'");
    //         to_stack.push(cargo_crate);
    //         print_stacks(&stacks);
    //     }
    // }

    get_top_of_stacks(&stacks).to_string()
}

pub fn solve_part_2() -> String {
    "TODO".to_string()
}

fn print_stacks(stacks:&Vec<Vec<String>>) {
    let mut i = 0;
    for stack in stacks {
        i += 1;
        let mut s = "".to_string();
        for char in stack {
            s += char.to_string().as_str();
        }
        println!("{i}: {s}");
    }
}

fn get_top_of_stacks(stacks:&Vec<Vec<String>>) -> String {
    stacks.iter().fold(
        "".to_string(),
        |top: String, stack| top + stack.last().unwrap().to_string().as_str()
    )
}

fn get_input() -> (Vec<Vec<String>>, Vec<(u32, u32, u32)>) {
    let mut stacks:Vec<Vec<String>> = Vec::new();
    let mut commands:Vec<(u32, u32, u32)> = Vec::new();
    let input_rows = crate::utils::load_input_list(5, "test");

    let mut number_row = 0;
    for row in &input_rows {
        if row.is_empty() {
            break;
        }
        number_row += 1;
    }

    for i in (0..number_row-1).rev() {
        let row = input_rows.get(i).unwrap();
        println!("{i}: {row}");
        let mut stack_nr = 0;
        let parts = split_n(row, 4);
        for part in parts {
            let part = part.replace("[", "").replace("]", "");
            let part = part.trim().to_string();
            println!("{stack_nr} => '{part}'");
            if !part.is_empty() {
                stacks[stack_nr].push(part);
            }
            stack_nr += 1;
        }
    }

    let command_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for row in &input_rows {
        for cap in command_regex.captures_iter(row.as_str()) {
            commands.push((
                cap[1].parse::<u32>().unwrap(),
                cap[2].parse::<u32>().unwrap()-1,
                cap[3].parse::<u32>().unwrap()-1,
            ));
        }
    }

    (stacks, commands)
}

fn split_n(s:&String, n:usize) -> Vec<String> {
    let mut result:Vec<String> = Vec::new();
    let mut part = "".to_string();
    for (i, c) in s.chars().enumerate() {
        part.push(c);
        if (i + 1) % n == 0 {
            result.push(part);
            part = "".to_string();
        }
    }
    result.push(part);
    result
}
