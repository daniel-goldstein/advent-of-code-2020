use crate::utils::read_input_to_string;
use regex::Regex;
use std::mem;
use std::collections::HashMap;

type Program = Vec<MaskProcedure>;
type Mask = Vec<Option<bool>>; // Wear one!

struct MaskProcedure {
    mask: Mask,
    writes: Vec<(u64, u64)>,
}

pub fn day14() {
    let mut memory: HashMap<u64, u64> = HashMap::new();

}

// ugly but I got to use mem::take so that was fun
fn read_input() -> Program {
    let mem_regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();

    let input = read_input_to_string(String::from("day12"));
    let mut program = vec![];
    let mut mask: Option<Mask> = None;
    let mut writes = vec![];

    for line in input.lines() {
        match line.strip_prefix("mask = ") {
            Some(mask_str) => {
                if mask.is_some() {
                    program.push(MaskProcedure {
                        mask: mem::take(&mut mask).unwrap(),
                        writes: mem::take(&mut writes),
                    });
                }
                mask = Some(
                    mask_str
                        .chars()
                        .map(|c| match c {
                            'x' => None,
                            '1' => Some(true),
                            '0' => Some(false),
                            _ => panic!("Eoh no"),
                        })
                        .collect::<Mask>(),
                );
            }
            None => {
                let cap = mem_regex.captures(line).unwrap();
                writes.push((
                    cap[0].parse::<u64>().unwrap(),
                    cap[1].parse::<u64>().unwrap(),
                ));
            }
        }
    }

    program
}
