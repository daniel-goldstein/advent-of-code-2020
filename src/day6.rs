use crate::utils::read_input_to_string;
use std::collections::HashSet;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub fn day6() {
    println!("DAY 5");

    let input = read_input_to_string(String::from("day6"));
    let counts: usize = input.split("\n\n").map(count_unique_any_answer).sum();
    println!("The total number of questions is: {}", counts);

    let counts: usize = input.split("\n\n").map(count_unique_all_answer).sum();
    println!("Actually, the total number of questions is: {}", counts);
}

fn count_unique_any_answer(group_answers: &str) -> usize {
    group_answers
        .lines()
        .flat_map(|s| s.chars())
        .collect::<HashSet<char>>()
        .len()
}

fn count_unique_all_answer(group_answers: &str) -> usize {
    ASCII_LOWER
        .iter()
        .filter(|&c| group_answers.lines().all(|s| s.find(*c).is_some()))
        .count()
}
