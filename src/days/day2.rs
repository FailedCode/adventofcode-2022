use itertools::Itertools;
use std::collections::HashMap;

// A/X - Rock - 1
// B/Y - Paper - 2
// C/Z - Scissors - 3
// lose: 0
// draw: 3
// win: 6
pub fn solve_part_1() -> String {
    let input_rows = crate::utils::load_input_list(2, "day");
    let mut points:u32 = 0;
    let mut scores_play = HashMap::new();
    scores_play.insert(String::from("X"), 1);
    scores_play.insert(String::from("Y"), 2);
    scores_play.insert(String::from("Z"), 3);

    let mut scores_outcome = HashMap::new();
    scores_outcome.insert(String::from("loss"), 0);
    scores_outcome.insert(String::from("draw"), 3);
    scores_outcome.insert(String::from("win"), 6);

    let mut outcomes = HashMap::new();
    outcomes.insert(String::from("AX"), String::from("draw"));
    outcomes.insert(String::from("BY"), String::from("draw"));
    outcomes.insert(String::from("CZ"), String::from("draw"));
    outcomes.insert(String::from("AY"), String::from("win"));
    outcomes.insert(String::from("BZ"), String::from("win"));
    outcomes.insert(String::from("CX"), String::from("win"));
    outcomes.insert(String::from("AZ"), String::from("loss"));
    outcomes.insert(String::from("BX"), String::from("loss"));
    outcomes.insert(String::from("CY"), String::from("loss"));

    for row in input_rows {
        if let Some((move_enemy, move_myself)) = row.split_whitespace().collect_tuple()  {
            let outcome = outcomes.get(&[move_enemy, move_myself].join("")).unwrap();
            points += scores_play.get(move_myself).copied().unwrap_or(0);
            points += scores_outcome.get(outcome).copied().unwrap_or(0);
        }
    }
    points.to_string()
}

// X - lose
// Y - draw
// Z - win
pub fn solve_part_2() -> String {
    let input_rows = crate::utils::load_input_list(2, "day");
    let mut points:u32 = 0;

    let mut scores_play = HashMap::new();
    scores_play.insert(String::from("A"), 1);
    scores_play.insert(String::from("B"), 2);
    scores_play.insert(String::from("C"), 3);

    let mut scores_outcome = HashMap::new();
    scores_outcome.insert(String::from("X"), 0);
    scores_outcome.insert(String::from("Y"), 3);
    scores_outcome.insert(String::from("Z"), 6);

    let mut moves = HashMap::new();
    moves.insert(String::from("AX"), String::from("C"));
    moves.insert(String::from("AY"), String::from("A"));
    moves.insert(String::from("AZ"), String::from("B"));
    moves.insert(String::from("BX"), String::from("A"));
    moves.insert(String::from("BY"), String::from("B"));
    moves.insert(String::from("BZ"), String::from("C"));
    moves.insert(String::from("CX"), String::from("B"));
    moves.insert(String::from("CY"), String::from("C"));
    moves.insert(String::from("CZ"), String::from("A"));

    for row in input_rows {
        if let Some((move_enemy, outcome)) = row.split_whitespace().collect_tuple()  {
            let move_myself = moves.get(&[move_enemy, outcome].join("")).unwrap();
            points += scores_play.get(move_myself).copied().unwrap_or(0);
            points += scores_outcome.get(outcome).copied().unwrap_or(0);
        }
    }
    points.to_string()
}
