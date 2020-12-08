use crate::utils::read_input_to_string;

pub fn day8() {
    println!("DAY 8");
    println!("The value of the accumulator is: {}", run());
}

fn run() -> i32 {
    let input = read_input_to_string(String::from("day8"));
    let mut program = read_program(&input);
    let mut accumulator: i32 = 0;
    let mut idx = 0;
    loop {
        println!("The accumulator is: {}", accumulator);
        let (instruction, val, executed) = program[idx];
        if executed {
            return accumulator;
        }
        program[idx].2 = true;
        match instruction {
            "nop" => idx += 1,
            "jmp" => idx = ((idx as i32) + val) as usize,
            "acc" => {
                accumulator += val;
                idx += 1;
            }
            _ => panic!("Unknown instruction: {}", instruction)
        }
    }
}

fn read_program(input: &str) -> Vec<(&str, i32, bool)> {
    input
        .lines()
        .map(|line| {
            let mut line_iter = line.split_whitespace();
            (
                line_iter.next().unwrap(),
                line_iter.next().unwrap().parse::<i32>().unwrap(),
                false,
            )
        })
        .collect()
}
