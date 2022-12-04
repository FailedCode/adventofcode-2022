use regex::Regex;

pub fn solve_part_1() -> String {
    let mut contain_sum: u32 = 0;
    let ranges = get_input_ranges();
    for range in ranges {
        let r1s  = range.get(0).unwrap();
        let r1e  = range.get(1).unwrap();
        let r2s  = range.get(2).unwrap();
        let r2e  = range.get(3).unwrap();
        if (r2s >= r1s && r2e <= r1e) || (r1s >= r2s && r1e <= r2e) {
            contain_sum += 1;
        }
    }
    contain_sum.to_string()
}

pub fn solve_part_2() -> String {
    "TODO: implement".to_string()
}

fn get_input_ranges() -> Vec<Vec<u32>> {
    let mut ranges:Vec<Vec<u32>> = Vec::new();
    let input_rows = crate::utils::load_input_list(4, "day");
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    for row in input_rows {
        for cap in re.captures_iter(row.as_str()) {
            ranges.push(vec![
                cap[1].parse::<u32>().unwrap(),
                cap[2].parse::<u32>().unwrap(),
                cap[3].parse::<u32>().unwrap(),
                cap[4].parse::<u32>().unwrap()
            ]);
        }
    }
    ranges
}
