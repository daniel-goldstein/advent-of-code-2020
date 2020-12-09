use crate::day1::find_sum_to;
use crate::utils::read_input_to_string;

pub fn day9() {
    println!("DAY 9");

    let nums: Vec<i64> = read_input_to_string(String::from("day9"))
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    println!(
        "First invalid number is: {}",
        find_first_invalid_number(&nums)
    );
    println!("Numbers that sum to valid number are: {}", part2(&nums));
}

fn find_first_invalid_number(nums: &[i64]) -> i64 {
    let preamble_len = 25;
    let mut idx = preamble_len;
    while idx < nums.len() {
        if find_sum_to(&nums[idx - preamble_len..idx], nums[idx]).is_none() {
            return nums[idx];
        }
        idx += 1;
    }
    panic!("Invalid number not found");
}

fn part2(nums: &[i64]) -> i64 {
    let prefix_sums: Vec<i64> = (1..nums.len()).fold(vec![nums[0]], |mut sums, i| {
        sums.push(nums[i] + sums[i - 1]);
        return sums;
    });

    let invalid = find_first_invalid_number(&nums);
    let (i, j) = find_difference_to(&prefix_sums, invalid).unwrap();
    return nums[i..=j].iter().min().unwrap() + nums[i..=j].iter().max().unwrap();
}

fn find_difference_to(nums: &[i64], n: i64) -> Option<(usize, usize)> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[j] - nums[i] == n {
                return Some((i + 1, j));
            }
        }
    }

    None
}
