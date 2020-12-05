use crate::utils::read_input_to_string;
use std::cmp::Ordering;
use std::ops::Range;

pub fn day5() {
    println!("DAY 5");
    let input = read_input_to_string(String::from("day5"));
    println!(
        "The max seat id is: {}",
        input.lines().map(find_seat_id).max().unwrap()
    );

    let input = read_input_to_string(String::from("day5"));
    let mut seats: Vec<usize> = input.lines().map(find_seat_id).collect();
    seats.sort();
    println!(
        "My seat id is: {}",
        seats
            .iter()
            .zip(seats.iter().skip(1))
            .find(|(&left, &right)| left + 2usize == right)
            .unwrap()
            .0
            + 1
    );
}

fn find_seat_id(boarding_pass: &str) -> usize {
    let (forward_back, left_right) = boarding_pass.split_at(7);
    let row = bin_search(0..128, 'B', 'F', forward_back);
    let col = bin_search(0..8, 'R', 'L', left_right);
    row * 8 + col
}

fn bin_search(range: Range<u32>, less: char, greater: char, path: &str) -> usize {
    let mut dirs = path.chars();
    range
        .collect::<Vec<u32>>()
        .binary_search_by(move |_| match dirs.next() {
            Some(c) if c == less => Ordering::Less,
            Some(c) if c == greater => Ordering::Greater,
            None => Ordering::Equal,
            _ => panic!("Well that's not good"),
        })
        .unwrap()
}
