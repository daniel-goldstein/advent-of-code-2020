use crate::utils::read_input_to_string;
use std::collections::HashSet;

pub fn day8() {
    println!("DAY 8");
    println!("The value of the accumulator is: {}", part1());
    println!("The value of the accumulator after fixing is: {}", part2());
}

fn part1() -> i32 {
    let input = read_input_to_string(String::from("day8"));
    let program = read_program(&input);
    let mut accumulator: i32 = 0;
    let mut idx = 0;
    let mut executed = HashSet::new();
    loop {
        let (instruction, val) = program[idx];
        if executed.contains(&idx) {
            return accumulator;
        }
        executed.insert(idx);
        match instruction {
            "nop" => idx += 1,
            "jmp" => idx = ((idx as i32) + val) as usize,
            "acc" => {
                accumulator += val;
                idx += 1;
            }
            _ => panic!("Unknown instruction: {}", instruction),
        }
    }
}

fn part2() -> i32 {
    let input = read_input_to_string(String::from("day8"));
    let mut program = read_program(&input);
    let mut line = 0;
    while line < program.len() {
        match program[line].0 {
            "jmp" => {
                program[line].0 = "nop";
                if let Some(acc) = run_program(&program) {
                    return acc;
                }
                program[line].0 = "jmp";
            }
            "nop" => {
                program[line].0 = "jmp";
                if let Some(acc) = run_program(&program) {
                    return acc;
                }
                program[line].0 = "nop";
            }
            _ => {}
        }
        line += 1;
    }

    panic!("Nothing worked!");
}

fn run_program(program: &Vec<(&str, i32)>) -> Option<i32> {
    let mut accumulator: i32 = 0;
    let mut idx = 0;
    let mut executed = HashSet::new();
    while idx < program.len() {
        let (instruction, val) = program[idx];
        if executed.contains(&idx) {
            return None;
        }
        executed.insert(idx);
        match instruction {
            "nop" => idx += 1,
            "jmp" => idx = ((idx as i32) + val) as usize,
            "acc" => {
                accumulator += val;
                idx += 1;
            }
            _ => panic!("Unknown instruction: {}", instruction),
        }
    }

    Some(accumulator)
}

fn read_program(input: &str) -> Vec<(&str, i32)> {
    input
        .lines()
        .map(|line| {
            let mut line_iter = line.split_whitespace();
            (
                line_iter.next().unwrap(),
                line_iter.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect()
}
