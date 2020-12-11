use crate::utils::read_input_to_string;

pub fn day10() {
    let mut jolts: Vec<u32> = read_input_to_string(String::from("day10"))
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    jolts.sort();
    jolts.insert(0, 0);
    jolts.push(jolts.last().unwrap() + 3);

    println!("Jolts: {}", part1(&jolts));
    println!("# arrangements: {}", part2(&jolts));
}

fn part1(jolts: &[u32]) -> usize {
    let deltas: Vec<u32> = jolts
        .iter()
        .skip(1)
        .zip(jolts.iter())
        .map(|(j2, j1)| j2 - j1)
        .collect();

    let ones = deltas.iter().filter(|j| **j == 1).count();
    let threes = deltas.iter().filter(|j| **j == 3).count();
    ones * threes
}

fn part2(jolts: &[u32]) -> usize {
    let mut arrangements_ending_in = vec![1];
    for i in 1..jolts.len() {
        let mut ending_at_i = 0;
        for j in 0..i {
            if jolts[i] - jolts[j] <= 3 {
                ending_at_i += arrangements_ending_in[j];
            }
        }
        arrangements_ending_in.push(ending_at_i);
    }

    *arrangements_ending_in.last().unwrap()
}
