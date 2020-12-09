use crate::utils::read_input_to_string;

pub fn day1() {
    println!("DAY 1");

    let input = read_input();
    let (x, y) = find_sum_to(&input, 2020).unwrap();
    println!("The answer is: {}", x * y);

    let input = read_input();
    let (x, y, z) = find_three_sum_to(&input, 2020);
    println!("The answer is: {}", x * y * z);
}

pub fn find_sum_to(nums: &[i64], sum: i64) -> Option<(i64, i64)> {
    for &i in nums.iter() {
        for &j in nums.iter() {
            if i + j == sum {
                return Some((i, j));
            }
        }
    }
    None
}

fn find_three_sum_to(nums: &[i64], sum: i64) -> (i64, i64, i64) {
    for &i in nums.iter() {
        if let Some((j, k)) = find_sum_to(nums, sum - i) {
            return (i, j, k);
        }
    }
    panic!("Couldn't find desired numbers");
}

fn read_input() -> Vec<i64> {
    read_input_to_string(String::from("day1"))
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}
