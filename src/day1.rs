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

fn find_sum_to(nums: &Vec<i32>, sum: i32) -> Option<(i32, i32)> {
    for &i in nums.iter() {
        for &j in nums.iter() {
            if i + j == sum {
                return Some((i, j));
            }
        }
    }
    None
}

fn find_three_sum_to(nums: &Vec<i32>, sum: i32) -> (i32, i32, i32) {
    for &i in nums.iter() {
        if let Some((j, k)) = find_sum_to(nums, sum - i) {
            return (i, j, k);
        }
    }
    panic!("Couldn't find desired numbers");
}

fn read_input() -> Vec<i32> {
    read_input_to_string(String::from("day1"))
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}
